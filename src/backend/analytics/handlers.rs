use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Json,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::backend::{
    entities::{link_click, shortened_link},
    middleware::AuthUser,
    repositories::{ShortenedLinkRepository, WorkspaceMemberRepository, WorkspaceRepository},
};
use crate::shared::responses::ApiError;

pub type ErrorResponse = ApiError;

// ===== Query Parameters =====

#[derive(Debug, Deserialize)]
pub struct AnalyticsQuery {
    pub workspace_id: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LinkAnalyticsQuery {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

// ===== Response DTOs =====

#[derive(Debug, Serialize)]
pub struct AnalyticsOverviewResponse {
    pub total_clicks: u64,
    pub clicks_today: u64,
    pub clicks_this_week: u64,
    pub clicks_this_month: u64,
    pub top_links: Vec<TopLinkStats>,
}

#[derive(Debug, Serialize)]
pub struct TopLinkStats {
    pub link_id: String,
    pub short_code: String,
    pub title: Option<String>,
    pub original_url: String,
    pub click_count: u64,
}

#[derive(Debug, Serialize)]
pub struct ClicksByDateResponse {
    pub data: Vec<DateClickCount>,
}

#[derive(Debug, Serialize)]
pub struct DateClickCount {
    pub date: String,
    pub clicks: u64,
}

#[derive(Debug, Serialize)]
pub struct GeographicBreakdownResponse {
    pub countries: Vec<GeoStat>,
    pub cities: Vec<GeoStat>,
}

#[derive(Debug, Serialize)]
pub struct GeoStat {
    pub name: String,
    pub clicks: u64,
    pub percentage: f64,
}

#[derive(Debug, Serialize)]
pub struct ReferrerBreakdownResponse {
    pub referrers: Vec<ReferrerStat>,
}

#[derive(Debug, Serialize)]
pub struct ReferrerStat {
    pub source: String,
    pub clicks: u64,
    pub percentage: f64,
}

#[derive(Debug, Serialize)]
pub struct DeviceBreakdownResponse {
    pub browsers: Vec<DeviceStat>,
    pub devices: Vec<DeviceStat>,
}

#[derive(Debug, Serialize)]
pub struct DeviceStat {
    pub name: String,
    pub clicks: u64,
    pub percentage: f64,
}

#[derive(Debug, Serialize)]
pub struct LinkDetailedAnalyticsResponse {
    pub link_id: String,
    pub short_code: String,
    pub title: Option<String>,
    pub original_url: String,
    pub total_clicks: u64,
    pub clicks_by_date: Vec<DateClickCount>,
    pub geographic: GeographicBreakdownResponse,
    pub referrers: ReferrerBreakdownResponse,
    pub devices: DeviceBreakdownResponse,
}

// ===== Helper Functions =====

async fn verify_workspace_access(
    db: &DatabaseConnection,
    workspace_id: Uuid,
    user_id: Uuid,
) -> Result<(), (StatusCode, Json<ErrorResponse>)> {
    let ws = WorkspaceRepository::find_by_id(db, workspace_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: e.to_string(),
                }),
            )
        })?
        .ok_or((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Workspace non trouve".to_string(),
            }),
        ))?;

    if ws.owner_id != user_id {
        let member = WorkspaceMemberRepository::find_by_ids(db, workspace_id, user_id)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: e.to_string(),
                    }),
                )
            })?;
        if member.is_none() {
            return Err((
                StatusCode::FORBIDDEN,
                Json(ErrorResponse {
                    error: "Acces interdit".to_string(),
                }),
            ));
        }
    }

    Ok(())
}

fn parse_user_agent(user_agent: &str) -> (String, String) {
    let ua_lower = user_agent.to_lowercase();

    // Browser detection
    let browser = if ua_lower.contains("chrome") && !ua_lower.contains("edg") {
        "Chrome"
    } else if ua_lower.contains("firefox") {
        "Firefox"
    } else if ua_lower.contains("safari") && !ua_lower.contains("chrome") {
        "Safari"
    } else if ua_lower.contains("edg") {
        "Edge"
    } else if ua_lower.contains("opera") || ua_lower.contains("opr") {
        "Opera"
    } else {
        "Other"
    }
    .to_string();

    // Device detection
    let device = if ua_lower.contains("mobile") || ua_lower.contains("android") {
        "Mobile"
    } else if ua_lower.contains("tablet") || ua_lower.contains("ipad") {
        "Tablet"
    } else {
        "Desktop"
    }
    .to_string();

    (browser, device)
}

fn parse_referrer(referer: &str) -> String {
    if referer.is_empty() {
        return "Direct".to_string();
    }

    let referer_lower = referer.to_lowercase();

    if referer_lower.contains("google") {
        "Google"
    } else if referer_lower.contains("facebook") || referer_lower.contains("fb.com") {
        "Facebook"
    } else if referer_lower.contains("twitter") || referer_lower.contains("t.co") {
        "Twitter"
    } else if referer_lower.contains("linkedin") {
        "LinkedIn"
    } else if referer_lower.contains("instagram") {
        "Instagram"
    } else if referer_lower.contains("youtube") {
        "YouTube"
    } else if referer_lower.contains("reddit") {
        "Reddit"
    } else {
        // Extract domain from URL
        referer
            .split("://")
            .nth(1)
            .and_then(|s| s.split('/').next())
            .unwrap_or("Other")
    }
    .to_string()
}

// ===== Analytics Overview Handler =====

/// Handler pour obtenir une vue d'ensemble des analytics d'un workspace.
///
/// Retourne les statistiques globales : total de clics, clics aujourd'hui,
/// cette semaine, ce mois, et les top liens.
///
/// # Endpoint
///
/// `GET /api/analytics/overview?workspace_id=...&start_date=...&end_date=...`
///
/// # Arguments
///
/// * `auth_user` - Utilisateur authentifié
/// * `query` - Paramètres de requête (workspace_id requis, dates optionnelles)
///
/// # Returns
///
/// Vue d'ensemble des analytics avec statistiques agrégées.
///
/// # Errors
///
/// * `400 Bad Request` - workspace_id invalide
/// * `401 Unauthorized` - Token invalide
/// * `403 Forbidden` - L'utilisateur n'a pas accès au workspace
/// * `500 Internal Server Error` - Erreur de base de données
pub async fn get_analytics_overview_handler(
    auth_user: AuthUser,
    Query(query): Query<AnalyticsQuery>,
) -> Result<Json<AnalyticsOverviewResponse>, (StatusCode, Json<ErrorResponse>)> {
    let workspace_id = Uuid::parse_str(&query.workspace_id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "workspace_id invalide (UUID attendu)".to_string(),
            }),
        )
    })?;

    verify_workspace_access(&auth_user.state.db, workspace_id, auth_user.user_id).await?;

    // Get all links for this workspace
    let links = ShortenedLinkRepository::find_by_workspace(&auth_user.state.db, workspace_id, 10000, 0)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: e.to_string(),
                }),
            )
        })?;

    if links.is_empty() {
        return Ok(Json(AnalyticsOverviewResponse {
            total_clicks: 0,
            clicks_today: 0,
            clicks_this_week: 0,
            clicks_this_month: 0,
            top_links: vec![],
        }));
    }

    let link_ids: Vec<Uuid> = links.iter().map(|l| l.id).collect();

    // Get all clicks for these links
    let clicks = link_click::Entity::find()
        .filter(link_click::Column::LinkId.is_in(link_ids))
        .all(&auth_user.state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: e.to_string(),
                }),
            )
        })?;

    let now = chrono::Utc::now().naive_utc();
    let today_start = now.date().and_hms_opt(0, 0, 0).unwrap();
    let week_start = today_start - chrono::Duration::days(7);
    let month_start = today_start - chrono::Duration::days(30);

    let total_clicks = clicks.len() as u64;
    let mut clicks_today = 0u64;
    let mut clicks_this_week = 0u64;
    let mut clicks_this_month = 0u64;
    let mut click_counts: HashMap<Uuid, u64> = HashMap::new();

    for click in &clicks {
        if let Some(clicked_at) = click.clicked_at {
            if clicked_at >= today_start {
                clicks_today += 1;
            }
            if clicked_at >= week_start {
                clicks_this_week += 1;
            }
            if clicked_at >= month_start {
                clicks_this_month += 1;
            }
        }
        *click_counts.entry(click.link_id).or_insert(0) += 1;
    }

    // Get top 5 links
    let mut top_links: Vec<TopLinkStats> = links
        .iter()
        .map(|link| {
            let count = *click_counts.get(&link.id).unwrap_or(&0);
            TopLinkStats {
                link_id: link.id.to_string(),
                short_code: link.short_code.clone(),
                title: link.title.clone(),
                original_url: link.original_url.clone(),
                click_count: count,
            }
        })
        .collect();

    top_links.sort_by(|a, b| b.click_count.cmp(&a.click_count));
    top_links.truncate(5);

    Ok(Json(AnalyticsOverviewResponse {
        total_clicks,
        clicks_today,
        clicks_this_week,
        clicks_this_month,
        top_links,
    }))
}

// ===== Clicks by Date Handler =====

/// Handler pour obtenir les clics agrégés par date.
///
/// Retourne le nombre de clics par jour pour un workspace, avec filtres optionnels
/// par date de début et de fin.
///
/// # Endpoint
///
/// `GET /api/analytics/clicks-by-date?workspace_id=...&start_date=...&end_date=...`
///
/// # Arguments
///
/// * `auth_user` - Utilisateur authentifié
/// * `query` - Paramètres de requête (workspace_id requis, dates optionnelles au format YYYY-MM-DD)
///
/// # Returns
///
/// Liste des clics agrégés par date, triée chronologiquement.
///
/// # Errors
///
/// * `400 Bad Request` - workspace_id invalide ou format de date invalide
/// * `401 Unauthorized` - Token invalide
/// * `403 Forbidden` - L'utilisateur n'a pas accès au workspace
/// * `500 Internal Server Error` - Erreur de base de données
pub async fn get_clicks_by_date_handler(
    auth_user: AuthUser,
    Query(query): Query<AnalyticsQuery>,
) -> Result<Json<ClicksByDateResponse>, (StatusCode, Json<ErrorResponse>)> {
    let workspace_id = Uuid::parse_str(&query.workspace_id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "workspace_id invalide (UUID attendu)".to_string(),
            }),
        )
    })?;

    verify_workspace_access(&auth_user.state.db, workspace_id, auth_user.user_id).await?;

    let links = ShortenedLinkRepository::find_by_workspace(&auth_user.state.db, workspace_id, 10000, 0)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: e.to_string(),
                }),
            )
        })?;

    let link_ids: Vec<Uuid> = links.iter().map(|l| l.id).collect();

    let mut date_filter = link_click::Entity::find()
        .filter(link_click::Column::LinkId.is_in(link_ids));

    // Apply date filters if provided
    if let Some(start_date) = &query.start_date {
        if let Ok(start) = chrono::NaiveDate::parse_from_str(start_date, "%Y-%m-%d") {
            let start_dt = start.and_hms_opt(0, 0, 0).unwrap();
            date_filter = date_filter.filter(link_click::Column::ClickedAt.gte(start_dt));
        }
    }

    if let Some(end_date) = &query.end_date {
        if let Ok(end) = chrono::NaiveDate::parse_from_str(end_date, "%Y-%m-%d") {
            let end_dt = end.and_hms_opt(23, 59, 59).unwrap();
            date_filter = date_filter.filter(link_click::Column::ClickedAt.lte(end_dt));
        }
    }

    let clicks = date_filter
        .order_by_asc(link_click::Column::ClickedAt)
        .all(&auth_user.state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: e.to_string(),
                }),
            )
        })?;

    // Group clicks by date
    let mut date_counts: HashMap<String, u64> = HashMap::new();
    for click in clicks {
        if let Some(clicked_at) = click.clicked_at {
            let date_str = clicked_at.date().format("%Y-%m-%d").to_string();
            *date_counts.entry(date_str).or_insert(0) += 1;
        }
    }

    let mut data: Vec<DateClickCount> = date_counts
        .into_iter()
        .map(|(date, clicks)| DateClickCount { date, clicks })
        .collect();

    data.sort_by(|a, b| a.date.cmp(&b.date));

    Ok(Json(ClicksByDateResponse { data }))
}

// ===== Geographic Breakdown Handler =====

/// Handler pour obtenir le breakdown géographique des clics.
///
/// Retourne les statistiques de clics par pays et par ville pour un workspace.
/// Les résultats sont triés par nombre de clics décroissant et limités aux top 10.
///
/// # Endpoint
///
/// `GET /api/analytics/geographic?workspace_id=...&start_date=...&end_date=...`
///
/// # Arguments
///
/// * `auth_user` - Utilisateur authentifié
/// * `query` - Paramètres de requête (workspace_id requis, dates optionnelles)
///
/// # Returns
///
/// Breakdown géographique avec top 10 pays et top 10 villes, avec pourcentages.
///
/// # Errors
///
/// * `400 Bad Request` - workspace_id invalide
/// * `401 Unauthorized` - Token invalide
/// * `403 Forbidden` - L'utilisateur n'a pas accès au workspace
/// * `500 Internal Server Error` - Erreur de base de données
pub async fn get_geographic_breakdown_handler(
    auth_user: AuthUser,
    Query(query): Query<AnalyticsQuery>,
) -> Result<Json<GeographicBreakdownResponse>, (StatusCode, Json<ErrorResponse>)> {
    let workspace_id = Uuid::parse_str(&query.workspace_id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "workspace_id invalide (UUID attendu)".to_string(),
            }),
        )
    })?;

    verify_workspace_access(&auth_user.state.db, workspace_id, auth_user.user_id).await?;

    let links = ShortenedLinkRepository::find_by_workspace(&auth_user.state.db, workspace_id, 10000, 0)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: e.to_string(),
                }),
            )
        })?;

    let link_ids: Vec<Uuid> = links.iter().map(|l| l.id).collect();

    let clicks = link_click::Entity::find()
        .filter(link_click::Column::LinkId.is_in(link_ids))
        .all(&auth_user.state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: e.to_string(),
                }),
            )
        })?;

    let total = clicks.len() as f64;

    // Group by country
    let mut country_counts: HashMap<String, u64> = HashMap::new();
    let mut city_counts: HashMap<String, u64> = HashMap::new();

    for click in &clicks {
        let country = click.country.clone().unwrap_or_else(|| "Unknown".to_string());
        let city = click.city.clone().unwrap_or_else(|| "Unknown".to_string());
        *country_counts.entry(country).or_insert(0) += 1;
        *city_counts.entry(city).or_insert(0) += 1;
    }

    let mut countries: Vec<GeoStat> = country_counts
        .into_iter()
        .map(|(name, clicks)| GeoStat {
            name,
            clicks,
            percentage: if total > 0.0 {
                (clicks as f64 / total) * 100.0
            } else {
                0.0
            },
        })
        .collect();

    let mut cities: Vec<GeoStat> = city_counts
        .into_iter()
        .map(|(name, clicks)| GeoStat {
            name,
            clicks,
            percentage: if total > 0.0 {
                (clicks as f64 / total) * 100.0
            } else {
                0.0
            },
        })
        .collect();

    countries.sort_by(|a, b| b.clicks.cmp(&a.clicks));
    cities.sort_by(|a, b| b.clicks.cmp(&a.clicks));

    countries.truncate(10);
    cities.truncate(10);

    Ok(Json(GeographicBreakdownResponse { countries, cities }))
}

// ===== Referrer Breakdown Handler =====

/// Handler pour obtenir le breakdown par referrer (sources de trafic).
///
/// Analyse les headers HTTP "Referer" pour identifier les sources de trafic
/// (moteurs de recherche, réseaux sociaux, sites directs, etc.).
///
/// # Endpoint
///
/// `GET /api/analytics/referrers?workspace_id=...&start_date=...&end_date=...`
///
/// # Arguments
///
/// * `auth_user` - Utilisateur authentifié
/// * `query` - Paramètres de requête (workspace_id requis, dates optionnelles)
///
/// # Returns
///
/// Breakdown par referrer avec top 10 sources, triées par nombre de clics.
///
/// # Errors
///
/// * `400 Bad Request` - workspace_id invalide
/// * `401 Unauthorized` - Token invalide
/// * `403 Forbidden` - L'utilisateur n'a pas accès au workspace
/// * `500 Internal Server Error` - Erreur de base de données
pub async fn get_referrer_breakdown_handler(
    auth_user: AuthUser,
    Query(query): Query<AnalyticsQuery>,
) -> Result<Json<ReferrerBreakdownResponse>, (StatusCode, Json<ErrorResponse>)> {
    let workspace_id = Uuid::parse_str(&query.workspace_id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "workspace_id invalide (UUID attendu)".to_string(),
            }),
        )
    })?;

    verify_workspace_access(&auth_user.state.db, workspace_id, auth_user.user_id).await?;

    let links = ShortenedLinkRepository::find_by_workspace(&auth_user.state.db, workspace_id, 10000, 0)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: e.to_string(),
                }),
            )
        })?;

    let link_ids: Vec<Uuid> = links.iter().map(|l| l.id).collect();

    let clicks = link_click::Entity::find()
        .filter(link_click::Column::LinkId.is_in(link_ids))
        .all(&auth_user.state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: e.to_string(),
                }),
            )
        })?;

    let total = clicks.len() as f64;

    let mut referrer_counts: HashMap<String, u64> = HashMap::new();

    for click in &clicks {
        let source = parse_referrer(click.referer.as_deref().unwrap_or(""));
        *referrer_counts.entry(source).or_insert(0) += 1;
    }

    let mut referrers: Vec<ReferrerStat> = referrer_counts
        .into_iter()
        .map(|(source, clicks)| ReferrerStat {
            source,
            clicks,
            percentage: if total > 0.0 {
                (clicks as f64 / total) * 100.0
            } else {
                0.0
            },
        })
        .collect();

    referrers.sort_by(|a, b| b.clicks.cmp(&a.clicks));
    referrers.truncate(10);

    Ok(Json(ReferrerBreakdownResponse { referrers }))
}

// ===== Device Breakdown Handler =====

/// Handler pour obtenir le breakdown par device et navigateur.
///
/// Analyse les user-agents pour identifier les navigateurs et types d'appareils
/// (mobile, desktop, tablet) utilisés pour cliquer sur les liens.
///
/// # Endpoint
///
/// `GET /api/analytics/devices?workspace_id=...&start_date=...&end_date=...`
///
/// # Arguments
///
/// * `auth_user` - Utilisateur authentifié
/// * `query` - Paramètres de requête (workspace_id requis, dates optionnelles)
///
/// # Returns
///
/// Breakdown par navigateur et par type d'appareil, triés par nombre de clics.
///
/// # Errors
///
/// * `400 Bad Request` - workspace_id invalide
/// * `401 Unauthorized` - Token invalide
/// * `403 Forbidden` - L'utilisateur n'a pas accès au workspace
/// * `500 Internal Server Error` - Erreur de base de données
pub async fn get_device_breakdown_handler(
    auth_user: AuthUser,
    Query(query): Query<AnalyticsQuery>,
) -> Result<Json<DeviceBreakdownResponse>, (StatusCode, Json<ErrorResponse>)> {
    let workspace_id = Uuid::parse_str(&query.workspace_id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "workspace_id invalide (UUID attendu)".to_string(),
            }),
        )
    })?;

    verify_workspace_access(&auth_user.state.db, workspace_id, auth_user.user_id).await?;

    let links = ShortenedLinkRepository::find_by_workspace(&auth_user.state.db, workspace_id, 10000, 0)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: e.to_string(),
                }),
            )
        })?;

    let link_ids: Vec<Uuid> = links.iter().map(|l| l.id).collect();

    let clicks = link_click::Entity::find()
        .filter(link_click::Column::LinkId.is_in(link_ids))
        .all(&auth_user.state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: e.to_string(),
                }),
            )
        })?;

    let total = clicks.len() as f64;

    let mut browser_counts: HashMap<String, u64> = HashMap::new();
    let mut device_counts: HashMap<String, u64> = HashMap::new();

    for click in &clicks {
        let (browser, device) = parse_user_agent(click.user_agent.as_deref().unwrap_or(""));
        *browser_counts.entry(browser).or_insert(0) += 1;
        *device_counts.entry(device).or_insert(0) += 1;
    }

    let mut browsers: Vec<DeviceStat> = browser_counts
        .into_iter()
        .map(|(name, clicks)| DeviceStat {
            name,
            clicks,
            percentage: if total > 0.0 {
                (clicks as f64 / total) * 100.0
            } else {
                0.0
            },
        })
        .collect();

    let mut devices: Vec<DeviceStat> = device_counts
        .into_iter()
        .map(|(name, clicks)| DeviceStat {
            name,
            clicks,
            percentage: if total > 0.0 {
                (clicks as f64 / total) * 100.0
            } else {
                0.0
            },
        })
        .collect();

    browsers.sort_by(|a, b| b.clicks.cmp(&a.clicks));
    devices.sort_by(|a, b| b.clicks.cmp(&a.clicks));

    Ok(Json(DeviceBreakdownResponse { browsers, devices }))
}

// ===== Link Detailed Analytics Handler =====

/// Handler pour obtenir les analytics détaillés d'un lien spécifique.
///
/// Retourne une vue complète des analytics pour un lien : clics par date,
/// breakdown géographique, par device, et par referrer.
///
/// # Endpoint
///
/// `GET /api/analytics/links/{link_id}?start_date=...&end_date=...`
///
/// # Arguments
///
/// * `auth_user` - Utilisateur authentifié
/// * `link_id` - UUID du lien
/// * `query` - Paramètres de filtrage par date (optionnels)
///
/// # Returns
///
/// Analytics détaillés complets du lien.
///
/// # Errors
///
/// * `400 Bad Request` - link_id invalide
/// * `401 Unauthorized` - Token invalide
/// * `403 Forbidden` - L'utilisateur n'a pas accès au workspace du lien
/// * `404 Not Found` - Lien introuvable
/// * `500 Internal Server Error` - Erreur de base de données
pub async fn get_link_detailed_analytics_handler(
    auth_user: AuthUser,
    Path(link_id): Path<String>,
    Query(query): Query<LinkAnalyticsQuery>,
) -> Result<Json<LinkDetailedAnalyticsResponse>, (StatusCode, Json<ErrorResponse>)> {
    let link_uuid = Uuid::parse_str(&link_id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "link_id invalide (UUID attendu)".to_string(),
            }),
        )
    })?;

    // Get the link
    let link = shortened_link::Entity::find_by_id(link_uuid)
        .one(&auth_user.state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: e.to_string(),
                }),
            )
        })?
        .ok_or((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Lien non trouve".to_string(),
            }),
        ))?;

    // Verify access to this link's workspace
    verify_workspace_access(&auth_user.state.db, link.workspace_id, auth_user.user_id).await?;

    // Get clicks for this link
    let mut date_filter = link_click::Entity::find()
        .filter(link_click::Column::LinkId.eq(link_uuid));

    if let Some(start_date) = &query.start_date {
        if let Ok(start) = chrono::NaiveDate::parse_from_str(start_date, "%Y-%m-%d") {
            let start_dt = start.and_hms_opt(0, 0, 0).unwrap();
            date_filter = date_filter.filter(link_click::Column::ClickedAt.gte(start_dt));
        }
    }

    if let Some(end_date) = &query.end_date {
        if let Ok(end) = chrono::NaiveDate::parse_from_str(end_date, "%Y-%m-%d") {
            let end_dt = end.and_hms_opt(23, 59, 59).unwrap();
            date_filter = date_filter.filter(link_click::Column::ClickedAt.lte(end_dt));
        }
    }

    let clicks = date_filter
        .order_by_asc(link_click::Column::ClickedAt)
        .all(&auth_user.state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: e.to_string(),
                }),
            )
        })?;

    let total = clicks.len() as f64;

    // Process analytics data
    let mut date_counts: HashMap<String, u64> = HashMap::new();
    let mut country_counts: HashMap<String, u64> = HashMap::new();
    let mut city_counts: HashMap<String, u64> = HashMap::new();
    let mut referrer_counts: HashMap<String, u64> = HashMap::new();
    let mut browser_counts: HashMap<String, u64> = HashMap::new();
    let mut device_counts: HashMap<String, u64> = HashMap::new();

    for click in &clicks {
        // Date
        if let Some(clicked_at) = click.clicked_at {
            let date_str = clicked_at.date().format("%Y-%m-%d").to_string();
            *date_counts.entry(date_str).or_insert(0) += 1;
        }

        // Geographic
        let country = click.country.clone().unwrap_or_else(|| "Unknown".to_string());
        let city = click.city.clone().unwrap_or_else(|| "Unknown".to_string());
        *country_counts.entry(country).or_insert(0) += 1;
        *city_counts.entry(city).or_insert(0) += 1;

        // Referrer
        let source = parse_referrer(click.referer.as_deref().unwrap_or(""));
        *referrer_counts.entry(source).or_insert(0) += 1;

        // Device
        let (browser, device) = parse_user_agent(click.user_agent.as_deref().unwrap_or(""));
        *browser_counts.entry(browser).or_insert(0) += 1;
        *device_counts.entry(device).or_insert(0) += 1;
    }

    // Build response
    let mut clicks_by_date: Vec<DateClickCount> = date_counts
        .into_iter()
        .map(|(date, clicks)| DateClickCount { date, clicks })
        .collect();
    clicks_by_date.sort_by(|a, b| a.date.cmp(&b.date));

    let mut countries: Vec<GeoStat> = country_counts
        .into_iter()
        .map(|(name, clicks)| GeoStat {
            name,
            clicks,
            percentage: if total > 0.0 {
                (clicks as f64 / total) * 100.0
            } else {
                0.0
            },
        })
        .collect();
    countries.sort_by(|a, b| b.clicks.cmp(&a.clicks));
    countries.truncate(10);

    let mut cities: Vec<GeoStat> = city_counts
        .into_iter()
        .map(|(name, clicks)| GeoStat {
            name,
            clicks,
            percentage: if total > 0.0 {
                (clicks as f64 / total) * 100.0
            } else {
                0.0
            },
        })
        .collect();
    cities.sort_by(|a, b| b.clicks.cmp(&a.clicks));
    cities.truncate(10);

    let mut referrers: Vec<ReferrerStat> = referrer_counts
        .into_iter()
        .map(|(source, clicks)| ReferrerStat {
            source,
            clicks,
            percentage: if total > 0.0 {
                (clicks as f64 / total) * 100.0
            } else {
                0.0
            },
        })
        .collect();
    referrers.sort_by(|a, b| b.clicks.cmp(&a.clicks));

    let mut browsers: Vec<DeviceStat> = browser_counts
        .into_iter()
        .map(|(name, clicks)| DeviceStat {
            name,
            clicks,
            percentage: if total > 0.0 {
                (clicks as f64 / total) * 100.0
            } else {
                0.0
            },
        })
        .collect();
    browsers.sort_by(|a, b| b.clicks.cmp(&a.clicks));

    let mut devices: Vec<DeviceStat> = device_counts
        .into_iter()
        .map(|(name, clicks)| DeviceStat {
            name,
            clicks,
            percentage: if total > 0.0 {
                (clicks as f64 / total) * 100.0
            } else {
                0.0
            },
        })
        .collect();
    devices.sort_by(|a, b| b.clicks.cmp(&a.clicks));

    Ok(Json(LinkDetailedAnalyticsResponse {
        link_id: link.id.to_string(),
        short_code: link.short_code,
        title: link.title,
        original_url: link.original_url,
        total_clicks: clicks.len() as u64,
        clicks_by_date,
        geographic: GeographicBreakdownResponse { countries, cities },
        referrers: ReferrerBreakdownResponse { referrers },
        devices: DeviceBreakdownResponse { browsers, devices },
    }))
}
