mod page;
#[cfg(feature = "hydrate")]
mod client;

pub use page::AnalyticsPage;

#[cfg(feature = "hydrate")]
pub use client::{
    fetch_analytics_overview, fetch_clicks_by_date, fetch_device_breakdown,
    fetch_geographic_breakdown, fetch_link_analytics, fetch_referrer_breakdown,
};

#[derive(Clone, Debug)]
pub struct AnalyticsOverview {
    pub total_clicks: u64,
    pub clicks_today: u64,
    pub clicks_this_week: u64,
    pub clicks_this_month: u64,
    pub top_links: Vec<TopLinkStats>,
}

#[derive(Clone, Debug)]
pub struct TopLinkStats {
    pub link_id: String,
    pub short_code: String,
    pub title: Option<String>,
    pub original_url: String,
    pub click_count: u64,
}

#[derive(Clone, Debug)]
pub struct ClicksByDate {
    pub data: Vec<DateClickCount>,
}

#[derive(Clone, Debug)]
pub struct DateClickCount {
    pub date: String,
    pub clicks: u64,
}

#[derive(Clone, Debug)]
pub struct GeographicBreakdown {
    pub countries: Vec<GeoStat>,
    pub cities: Vec<GeoStat>,
}

#[derive(Clone, Debug)]
pub struct GeoStat {
    pub name: String,
    pub clicks: u64,
    pub percentage: f64,
}

#[derive(Clone, Debug)]
pub struct ReferrerBreakdown {
    pub referrers: Vec<ReferrerStat>,
}

#[derive(Clone, Debug)]
pub struct ReferrerStat {
    pub source: String,
    pub clicks: u64,
    pub percentage: f64,
}

#[derive(Clone, Debug)]
pub struct DeviceBreakdown {
    pub browsers: Vec<DeviceStat>,
    pub devices: Vec<DeviceStat>,
}

#[derive(Clone, Debug)]
pub struct DeviceStat {
    pub name: String,
    pub clicks: u64,
    pub percentage: f64,
}

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
