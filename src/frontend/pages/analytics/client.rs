//! Module client pour les appels API des analytics.
//!
//! Fournit les fonctions pour interagir avec l'API d'analytics
//! côté client (WASM).
//!
//! # Note
//!
//! Ce module est uniquement disponible avec la feature `hydrate`.

#[cfg(feature = "hydrate")]
use super::{
    AnalyticsOverview, ClicksByDate, DateClickCount, DeviceBreakdown, DeviceStat,
    GeoStat, GeographicBreakdown, LinkDetailedAnalytics, ReferrerBreakdown, ReferrerStat,
    TopLinkStats,
};
#[cfg(feature = "hydrate")]
use gloo_net::http::Request;

#[cfg(feature = "hydrate")]
#[derive(serde::Deserialize)]
struct AnalyticsOverviewResponse {
    total_clicks: u64,
    clicks_today: u64,
    clicks_this_week: u64,
    clicks_this_month: u64,
    top_links: Vec<TopLinkStatsResponse>,
}

#[cfg(feature = "hydrate")]
#[derive(serde::Deserialize)]
struct TopLinkStatsResponse {
    link_id: String,
    short_code: String,
    title: Option<String>,
    original_url: String,
    click_count: u64,
}

#[cfg(feature = "hydrate")]
#[derive(serde::Deserialize)]
struct ClicksByDateResponse {
    data: Vec<DateClickCountResponse>,
}

#[cfg(feature = "hydrate")]
#[derive(serde::Deserialize)]
struct DateClickCountResponse {
    date: String,
    clicks: u64,
}

#[cfg(feature = "hydrate")]
#[derive(serde::Deserialize)]
struct GeographicBreakdownResponse {
    countries: Vec<GeoStatResponse>,
    cities: Vec<GeoStatResponse>,
}

#[cfg(feature = "hydrate")]
#[derive(serde::Deserialize)]
struct GeoStatResponse {
    name: String,
    clicks: u64,
    percentage: f64,
}

#[cfg(feature = "hydrate")]
#[derive(serde::Deserialize)]
struct ReferrerBreakdownResponse {
    referrers: Vec<ReferrerStatResponse>,
}

#[cfg(feature = "hydrate")]
#[derive(serde::Deserialize)]
struct ReferrerStatResponse {
    source: String,
    clicks: u64,
    percentage: f64,
}

#[cfg(feature = "hydrate")]
#[derive(serde::Deserialize)]
struct DeviceBreakdownResponse {
    browsers: Vec<DeviceStatResponse>,
    devices: Vec<DeviceStatResponse>,
}

#[cfg(feature = "hydrate")]
#[derive(serde::Deserialize)]
struct DeviceStatResponse {
    name: String,
    clicks: u64,
    percentage: f64,
}

#[cfg(feature = "hydrate")]
#[derive(serde::Deserialize)]
struct LinkDetailedAnalyticsResponse {
    link_id: String,
    short_code: String,
    title: Option<String>,
    original_url: String,
    total_clicks: u64,
    clicks_by_date: Vec<DateClickCountResponse>,
    geographic: GeographicBreakdownResponse,
    referrers: ReferrerBreakdownResponse,
    devices: DeviceBreakdownResponse,
}

#[cfg(feature = "hydrate")]
#[derive(serde::Deserialize)]
struct ApiErrorResponse {
    error: String,
}

/// Récupère la vue d'ensemble des analytics d'un workspace.
///
/// # Arguments
///
/// * `workspace_id` - ID du workspace
/// * `token` - Token d'accès JWT
/// * `start_date` - Date de début optionnelle (format YYYY-MM-DD)
/// * `end_date` - Date de fin optionnelle (format YYYY-MM-DD)
///
/// # Returns
///
/// * `Ok(AnalyticsOverview)` - Vue d'ensemble des analytics
/// * `Err(String)` - En cas d'erreur
#[cfg(feature = "hydrate")]
pub async fn fetch_analytics_overview(
    workspace_id: &str,
    token: &str,
    start_date: Option<&str>,
    end_date: Option<&str>,
) -> Result<AnalyticsOverview, String> {
    let mut url = format!("/api/analytics/overview?workspace_id={}", workspace_id);
    if let Some(start) = start_date {
        url.push_str(&format!("&start_date={}", start));
    }
    if let Some(end) = end_date {
        url.push_str(&format!("&end_date={}", end));
    }

    let response = Request::get(&url)
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|err| err.to_string())?;

    if response.ok() {
        let parsed = response
            .json::<AnalyticsOverviewResponse>()
            .await
            .map_err(|err| format!("Reponse invalide: {err}"))?;

        Ok(AnalyticsOverview {
            total_clicks: parsed.total_clicks,
            clicks_today: parsed.clicks_today,
            clicks_this_week: parsed.clicks_this_week,
            clicks_this_month: parsed.clicks_this_month,
            top_links: parsed
                .top_links
                .into_iter()
                .map(|l| TopLinkStats {
                    link_id: l.link_id,
                    short_code: l.short_code,
                    title: l.title,
                    original_url: l.original_url,
                    click_count: l.click_count,
                })
                .collect(),
        })
    } else {
        let status = response.status();
        let message = response
            .json::<ApiErrorResponse>()
            .await
            .map(|err| err.error)
            .unwrap_or_else(|_| format!("Erreur lors du chargement (code {})", status));
        Err(message)
    }
}

/// Récupère les clics agrégés par date.
///
/// # Arguments
///
/// * `workspace_id` - ID du workspace
/// * `token` - Token d'accès JWT
/// * `start_date` - Date de début optionnelle (format YYYY-MM-DD)
/// * `end_date` - Date de fin optionnelle (format YYYY-MM-DD)
///
/// # Returns
///
/// * `Ok(ClicksByDate)` - Clics agrégés par date
/// * `Err(String)` - En cas d'erreur
#[cfg(feature = "hydrate")]
pub async fn fetch_clicks_by_date(
    workspace_id: &str,
    token: &str,
    start_date: Option<&str>,
    end_date: Option<&str>,
) -> Result<ClicksByDate, String> {
    let mut url = format!("/api/analytics/clicks-by-date?workspace_id={}", workspace_id);
    if let Some(start) = start_date {
        url.push_str(&format!("&start_date={}", start));
    }
    if let Some(end) = end_date {
        url.push_str(&format!("&end_date={}", end));
    }

    let response = Request::get(&url)
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|err| err.to_string())?;

    if response.ok() {
        let parsed = response
            .json::<ClicksByDateResponse>()
            .await
            .map_err(|err| format!("Reponse invalide: {err}"))?;

        Ok(ClicksByDate {
            data: parsed
                .data
                .into_iter()
                .map(|d| DateClickCount {
                    date: d.date,
                    clicks: d.clicks,
                })
                .collect(),
        })
    } else {
        let status = response.status();
        let message = response
            .json::<ApiErrorResponse>()
            .await
            .map(|err| err.error)
            .unwrap_or_else(|_| format!("Erreur lors du chargement (code {})", status));
        Err(message)
    }
}

/// Récupère le breakdown géographique (pays et villes).
///
/// # Arguments
///
/// * `workspace_id` - ID du workspace
/// * `token` - Token d'accès JWT
///
/// # Returns
///
/// * `Ok(GeographicBreakdown)` - Breakdown géographique
/// * `Err(String)` - En cas d'erreur
#[cfg(feature = "hydrate")]
pub async fn fetch_geographic_breakdown(
    workspace_id: &str,
    token: &str,
) -> Result<GeographicBreakdown, String> {
    let url = format!("/api/analytics/geographic?workspace_id={}", workspace_id);

    let response = Request::get(&url)
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|err| err.to_string())?;

    if response.ok() {
        let parsed = response
            .json::<GeographicBreakdownResponse>()
            .await
            .map_err(|err| format!("Reponse invalide: {err}"))?;

        Ok(GeographicBreakdown {
            countries: parsed
                .countries
                .into_iter()
                .map(|g| GeoStat {
                    name: g.name,
                    clicks: g.clicks,
                    percentage: g.percentage,
                })
                .collect(),
            cities: parsed
                .cities
                .into_iter()
                .map(|g| GeoStat {
                    name: g.name,
                    clicks: g.clicks,
                    percentage: g.percentage,
                })
                .collect(),
        })
    } else {
        let status = response.status();
        let message = response
            .json::<ApiErrorResponse>()
            .await
            .map(|err| err.error)
            .unwrap_or_else(|_| format!("Erreur lors du chargement (code {})", status));
        Err(message)
    }
}

/// Récupère le breakdown par referrer (sources de trafic).
///
/// # Arguments
///
/// * `workspace_id` - ID du workspace
/// * `token` - Token d'accès JWT
///
/// # Returns
///
/// * `Ok(ReferrerBreakdown)` - Breakdown par referrer
/// * `Err(String)` - En cas d'erreur
#[cfg(feature = "hydrate")]
pub async fn fetch_referrer_breakdown(
    workspace_id: &str,
    token: &str,
) -> Result<ReferrerBreakdown, String> {
    let url = format!("/api/analytics/referrers?workspace_id={}", workspace_id);

    let response = Request::get(&url)
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|err| err.to_string())?;

    if response.ok() {
        let parsed = response
            .json::<ReferrerBreakdownResponse>()
            .await
            .map_err(|err| format!("Reponse invalide: {err}"))?;

        Ok(ReferrerBreakdown {
            referrers: parsed
                .referrers
                .into_iter()
                .map(|r| ReferrerStat {
                    source: r.source,
                    clicks: r.clicks,
                    percentage: r.percentage,
                })
                .collect(),
        })
    } else {
        let status = response.status();
        let message = response
            .json::<ApiErrorResponse>()
            .await
            .map(|err| err.error)
            .unwrap_or_else(|_| format!("Erreur lors du chargement (code {})", status));
        Err(message)
    }
}

/// Récupère le breakdown par device (navigateurs et appareils).
///
/// # Arguments
///
/// * `workspace_id` - ID du workspace
/// * `token` - Token d'accès JWT
///
/// # Returns
///
/// * `Ok(DeviceBreakdown)` - Breakdown par device
/// * `Err(String)` - En cas d'erreur
#[cfg(feature = "hydrate")]
pub async fn fetch_device_breakdown(
    workspace_id: &str,
    token: &str,
) -> Result<DeviceBreakdown, String> {
    let url = format!("/api/analytics/devices?workspace_id={}", workspace_id);

    let response = Request::get(&url)
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|err| err.to_string())?;

    if response.ok() {
        let parsed = response
            .json::<DeviceBreakdownResponse>()
            .await
            .map_err(|err| format!("Reponse invalide: {err}"))?;

        Ok(DeviceBreakdown {
            browsers: parsed
                .browsers
                .into_iter()
                .map(|d| DeviceStat {
                    name: d.name,
                    clicks: d.clicks,
                    percentage: d.percentage,
                })
                .collect(),
            devices: parsed
                .devices
                .into_iter()
                .map(|d| DeviceStat {
                    name: d.name,
                    clicks: d.clicks,
                    percentage: d.percentage,
                })
                .collect(),
        })
    } else {
        let status = response.status();
        let message = response
            .json::<ApiErrorResponse>()
            .await
            .map(|err| err.error)
            .unwrap_or_else(|_| format!("Erreur lors du chargement (code {})", status));
        Err(message)
    }
}

/// Récupère les analytics détaillés d'un lien spécifique.
///
/// # Arguments
///
/// * `link_id` - ID du lien
/// * `token` - Token d'accès JWT
/// * `start_date` - Date de début optionnelle (format YYYY-MM-DD)
/// * `end_date` - Date de fin optionnelle (format YYYY-MM-DD)
///
/// # Returns
///
/// * `Ok(LinkDetailedAnalytics)` - Analytics détaillés du lien
/// * `Err(String)` - En cas d'erreur
#[cfg(feature = "hydrate")]
pub async fn fetch_link_analytics(
    link_id: &str,
    token: &str,
    start_date: Option<&str>,
    end_date: Option<&str>,
) -> Result<LinkDetailedAnalytics, String> {
    let mut url = format!("/api/analytics/links/{}", link_id);
    let mut has_query = false;

    if let Some(start) = start_date {
        url.push_str(&format!("?start_date={}", start));
        has_query = true;
    }
    if let Some(end) = end_date {
        if has_query {
            url.push_str(&format!("&end_date={}", end));
        } else {
            url.push_str(&format!("?end_date={}", end));
        }
    }

    let response = Request::get(&url)
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|err| err.to_string())?;

    if response.ok() {
        let parsed = response
            .json::<LinkDetailedAnalyticsResponse>()
            .await
            .map_err(|err| format!("Reponse invalide: {err}"))?;

        Ok(LinkDetailedAnalytics {
            link_id: parsed.link_id,
            short_code: parsed.short_code,
            title: parsed.title,
            original_url: parsed.original_url,
            total_clicks: parsed.total_clicks,
            clicks_by_date: parsed
                .clicks_by_date
                .into_iter()
                .map(|d| DateClickCount {
                    date: d.date,
                    clicks: d.clicks,
                })
                .collect(),
            geographic: GeographicBreakdown {
                countries: parsed
                    .geographic
                    .countries
                    .into_iter()
                    .map(|g| GeoStat {
                        name: g.name,
                        clicks: g.clicks,
                        percentage: g.percentage,
                    })
                    .collect(),
                cities: parsed
                    .geographic
                    .cities
                    .into_iter()
                    .map(|g| GeoStat {
                        name: g.name,
                        clicks: g.clicks,
                        percentage: g.percentage,
                    })
                    .collect(),
            },
            referrers: ReferrerBreakdown {
                referrers: parsed
                    .referrers
                    .referrers
                    .into_iter()
                    .map(|r| ReferrerStat {
                        source: r.source,
                        clicks: r.clicks,
                        percentage: r.percentage,
                    })
                    .collect(),
            },
            devices: DeviceBreakdown {
                browsers: parsed
                    .devices
                    .browsers
                    .into_iter()
                    .map(|d| DeviceStat {
                        name: d.name,
                        clicks: d.clicks,
                        percentage: d.percentage,
                    })
                    .collect(),
                devices: parsed
                    .devices
                    .devices
                    .into_iter()
                    .map(|d| DeviceStat {
                        name: d.name,
                        clicks: d.clicks,
                        percentage: d.percentage,
                    })
                    .collect(),
            },
        })
    } else {
        let status = response.status();
        let message = response
            .json::<ApiErrorResponse>()
            .await
            .map(|err| err.error)
            .unwrap_or_else(|_| format!("Erreur lors du chargement (code {})", status));
        Err(message)
    }
}
