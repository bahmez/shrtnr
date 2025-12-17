//! Module d'analytics frontend.
//!
//! Fournit la page d'analytics et les structures de données pour visualiser
//! les statistiques des liens et workspaces.

mod page;
#[cfg(feature = "hydrate")]
mod client;

pub use page::AnalyticsPage;

#[cfg(feature = "hydrate")]
pub use client::{
    fetch_analytics_overview, fetch_clicks_by_date, fetch_device_breakdown,
    fetch_geographic_breakdown, fetch_link_analytics, fetch_referrer_breakdown,
};

/// Vue d'ensemble des analytics d'un workspace.
#[derive(Clone, Debug)]
pub struct AnalyticsOverview {
    pub total_clicks: u64,
    pub clicks_today: u64,
    pub clicks_this_week: u64,
    pub clicks_this_month: u64,
    pub top_links: Vec<TopLinkStats>,
}

/// Statistiques d'un lien dans le top.
#[derive(Clone, Debug)]
pub struct TopLinkStats {
    pub link_id: String,
    pub short_code: String,
    pub title: Option<String>,
    pub original_url: String,
    pub click_count: u64,
}

/// Clics agrégés par date.
#[derive(Clone, Debug)]
pub struct ClicksByDate {
    pub data: Vec<DateClickCount>,
}

/// Nombre de clics pour une date donnée.
#[derive(Clone, Debug)]
pub struct DateClickCount {
    pub date: String,
    pub clicks: u64,
}

/// Breakdown géographique (pays et villes).
#[derive(Clone, Debug)]
pub struct GeographicBreakdown {
    pub countries: Vec<GeoStat>,
    pub cities: Vec<GeoStat>,
}

/// Statistique géographique (pays ou ville).
#[derive(Clone, Debug)]
pub struct GeoStat {
    pub name: String,
    pub clicks: u64,
    pub percentage: f64,
}

/// Breakdown par referrer (sources de trafic).
#[derive(Clone, Debug)]
pub struct ReferrerBreakdown {
    pub referrers: Vec<ReferrerStat>,
}

/// Statistique d'un referrer.
#[derive(Clone, Debug)]
pub struct ReferrerStat {
    pub source: String,
    pub clicks: u64,
    pub percentage: f64,
}

/// Breakdown par device (navigateurs et appareils).
#[derive(Clone, Debug)]
pub struct DeviceBreakdown {
    pub browsers: Vec<DeviceStat>,
    pub devices: Vec<DeviceStat>,
}

/// Statistique d'un device (navigateur ou type d'appareil).
#[derive(Clone, Debug)]
pub struct DeviceStat {
    pub name: String,
    pub clicks: u64,
    pub percentage: f64,
}

/// Analytics détaillés d'un lien spécifique.
#[derive(Clone, Debug)]
pub struct LinkDetailedAnalytics {
    pub link_id: String,
    pub short_code: String,
    pub title: Option<String>,
    pub original_url: String,
    pub total_clicks: u64,
    pub clicks_by_date: Vec<DateClickCount>,
    pub geographic: GeographicBreakdown,
    pub referrers: ReferrerBreakdown,
    pub devices: DeviceBreakdown,
}
