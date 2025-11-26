use super::{
    AnalyticsOverview, ClicksByDate, DeviceBreakdown, GeographicBreakdown, LinkDetailedAnalytics,
    ReferrerBreakdown,
};
#[cfg(feature = "hydrate")]
use super::client::{
    fetch_analytics_overview, fetch_clicks_by_date, fetch_device_breakdown,
    fetch_geographic_breakdown, fetch_link_analytics, fetch_referrer_breakdown,
};
#[cfg(feature = "hydrate")]
use crate::frontend::design_system::{Badge, BadgeVariant};
use crate::frontend::{
    design_system::{
        Button, ButtonSize, ButtonVariant, Card, CardBody, CardHeader,
        FormControl, Heading, HeadingLevel, InputField, Text, TextTone,
    },
    layouts::DashboardLayout,
};
#[cfg(feature = "hydrate")]
use crate::frontend::state::{use_auth_store, use_workspace_store};
#[cfg(not(feature = "hydrate"))]
use crate::frontend::state::{use_workspace_store, WorkspaceSummary};
use leptos::prelude::*;
#[cfg(feature = "hydrate")]
use leptos::task::spawn_local;

// ===== Stat Card Component =====

#[component]
fn StatCard(
    #[prop(into)] label: String,
    #[prop(into)] value: String,
    #[prop(optional, into)] subtitle: Option<String>,
    #[prop(optional, into)] icon: Option<&'static str>,
) -> impl IntoView {
    view! {
        <Card>
            <CardHeader class="justify-center min-h-[120px] pb-6">
                <div class="flex items-center gap-3">
                    {icon.map(|i| view! {
                        <span class="text-2xl text-brand/60">{i}</span>
                    })}
                    <Text tone=TextTone::Muted class="text-xs uppercase tracking-wide">
                        {label}
                    </Text>
                </div>
                <Heading level=HeadingLevel::H2 class="text-3xl">
                    {value}
                </Heading>
                {subtitle.map(|s| view! {
                    <Text tone=TextTone::Subtle class="text-xs">{s}</Text>
                })}
            </CardHeader>
        </Card>
    }
}

// ===== Progress Bar Component =====

#[component]
fn ProgressBar(
    #[prop(into)] percentage: f64,
    #[prop(optional, into)] color: Option<String>,
) -> impl IntoView {
    let width = format!("{}%", percentage.min(100.0));
    let bg_color = color.unwrap_or_else(|| "bg-brand".to_string());

    view! {
        <div class="h-2 w-full rounded-full bg-surface-strong overflow-hidden">
            <div
                class=format!("h-full rounded-full transition-all duration-500 {}", bg_color)
                style=format!("width: {}", width)
            />
        </div>
    }
}

// ===== Overview Section =====

#[cfg(feature = "hydrate")]
fn overview_section(
    overview: RwSignal<Option<AnalyticsOverview>>,
    loading: RwSignal<bool>,
    error: RwSignal<Option<String>>,
) -> impl IntoView {
    view! {
        <div class="grid gap-6 md:grid-cols-2 lg:grid-cols-4">
            {move || {
                if loading.get() {
                    view! {
                        <StatCard label="Total clics" value="..." />
                        <StatCard label="Aujourd'hui" value="..." />
                        <StatCard label="Cette semaine" value="..." />
                        <StatCard label="Ce mois" value="..." />
                    }.into_any()
                } else if let Some(data) = overview.get() {
                    view! {
                        <StatCard
                            label="Total clics"
                            value=data.total_clicks.to_string()
                            icon="click"
                        />
                        <StatCard
                            label="Aujourd'hui"
                            value=data.clicks_today.to_string()
                            subtitle="derniere 24h"
                        />
                        <StatCard
                            label="Cette semaine"
                            value=data.clicks_this_week.to_string()
                            subtitle="7 derniers jours"
                        />
                        <StatCard
                            label="Ce mois"
                            value=data.clicks_this_month.to_string()
                            subtitle="30 derniers jours"
                        />
                    }.into_any()
                } else {
                    view! {
                        <StatCard label="Total clics" value="-" />
                        <StatCard label="Aujourd'hui" value="-" />
                        <StatCard label="Cette semaine" value="-" />
                        <StatCard label="Ce mois" value="-" />
                    }.into_any()
                }
            }}
        </div>

        {move || error.get().map(|err| view! {
            <div class="rounded-xl border border-danger/50 bg-danger/10 px-4 py-3 text-sm text-danger">
                {format!("Erreur: {}", err)}
            </div>
        })}
    }
}

#[cfg(not(feature = "hydrate"))]
fn overview_section(
    _overview: RwSignal<Option<AnalyticsOverview>>,
    _loading: RwSignal<bool>,
    _error: RwSignal<Option<String>>,
) -> impl IntoView {
    view! {
        <div class="grid gap-6 md:grid-cols-2 lg:grid-cols-4">
            <StatCard label="Total clics" value="..." />
            <StatCard label="Aujourd'hui" value="..." />
            <StatCard label="Cette semaine" value="..." />
            <StatCard label="Ce mois" value="..." />
        </div>
    }
}

// ===== Top Links Section =====

#[cfg(feature = "hydrate")]
fn top_links_section(
    overview: RwSignal<Option<AnalyticsOverview>>,
    on_select_link: Callback<String>,
) -> impl IntoView {
    view! {
        <Card>
            <CardHeader>
                <div class="flex items-center justify-between">
                    <Text tone=TextTone::Muted class="text-xs uppercase tracking-wide">
                        "Top liens"
                    </Text>
                    <Badge variant=BadgeVariant::Subtle>"Top 5"</Badge>
                </div>
            </CardHeader>
            <CardBody>
                {move || {
                    let data = overview.get();
                    if let Some(overview_data) = data {
                        if overview_data.top_links.is_empty() {
                            view! {
                                <div class="py-8 text-center">
                                    <Text tone=TextTone::Muted>"Aucun clic enregistre"</Text>
                                </div>
                            }.into_any()
                        } else {
                            let max_clicks = overview_data.top_links.iter().map(|l| l.click_count).max().unwrap_or(1);
                            overview_data.top_links.into_iter().map(|link| {
                                let link_id = link.link_id.clone();
                                let on_select = on_select_link.clone();
                                let percentage = (link.click_count as f64 / max_clicks as f64) * 100.0;
                                view! {
                                    <button
                                        class="w-full text-left py-3 px-2 rounded-lg hover:bg-surface-strong/50 transition group"
                                        on:click=move |_| on_select.run(link_id.clone())
                                    >
                                        <div class="flex items-center justify-between mb-2">
                                            <div class="flex-1 min-w-0 pr-4">
                                                <Text class="font-medium text-sm truncate block">
                                                    {link.title.clone().unwrap_or_else(|| link.short_code.clone())}
                                                </Text>
                                                <Text tone=TextTone::Subtle class="text-xs truncate block">
                                                    {link.original_url.clone()}
                                                </Text>
                                            </div>
                                            <div class="flex items-center gap-2">
                                                <Badge variant=BadgeVariant::Outline>
                                                    {format!("{} clics", link.click_count)}
                                                </Badge>
                                                <span class="text-foreground/30 group-hover:text-foreground/60 transition">
                                                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                                        <path d="m9 18 6-6-6-6"/>
                                                    </svg>
                                                </span>
                                            </div>
                                        </div>
                                        <ProgressBar percentage=percentage />
                                    </button>
                                }
                            }).collect_view().into_any()
                        }
                    } else {
                        view! {
                            <div class="py-8 text-center">
                                <Text tone=TextTone::Muted>"Chargement..."</Text>
                            </div>
                        }.into_any()
                    }
                }}
            </CardBody>
        </Card>
    }
}

#[cfg(not(feature = "hydrate"))]
fn top_links_section(
    _overview: RwSignal<Option<AnalyticsOverview>>,
    _on_select_link: Callback<String>,
) -> impl IntoView {
    view! {
        <Card>
            <CardHeader>
                <Text tone=TextTone::Muted class="text-xs uppercase tracking-wide">
                    "Top liens"
                </Text>
            </CardHeader>
            <CardBody>
                <div class="py-8 text-center">
                    <Text tone=TextTone::Muted>"Chargement..."</Text>
                </div>
            </CardBody>
        </Card>
    }
}

// ===== Clicks Timeline Section =====

#[cfg(feature = "hydrate")]
fn clicks_timeline_section(
    clicks_by_date: RwSignal<Option<ClicksByDate>>,
    loading: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <Card>
            <CardHeader>
                <Text tone=TextTone::Muted class="text-xs uppercase tracking-wide">
                    "Clics par jour"
                </Text>
            </CardHeader>
            <CardBody class="pt-4">
                {move || {
                    if loading.get() {
                        view! {
                            <div class="py-8 text-center">
                                <Text tone=TextTone::Muted>"Chargement..."</Text>
                            </div>
                        }.into_any()
                    } else if let Some(data) = clicks_by_date.get() {
                        if data.data.is_empty() {
                            view! {
                                <div class="py-8 text-center">
                                    <Text tone=TextTone::Muted>"Aucune donnee disponible"</Text>
                                </div>
                            }.into_any()
                        } else {
                            let max_clicks = data.data.iter().map(|d| d.clicks).max().unwrap_or(1);
                            view! {
                                <div class="space-y-2 max-h-[300px] overflow-y-auto">
                                    {data.data.into_iter().rev().take(14).map(|day| {
                                        let percentage = (day.clicks as f64 / max_clicks as f64) * 100.0;
                                        view! {
                                            <div class="flex items-center gap-4">
                                                <Text tone=TextTone::Subtle class="text-xs w-24 text-right font-mono">
                                                    {day.date.clone()}
                                                </Text>
                                                <div class="flex-1">
                                                    <ProgressBar percentage=percentage />
                                                </div>
                                                <Text class="text-sm w-16 text-right font-medium">
                                                    {day.clicks.to_string()}
                                                </Text>
                                            </div>
                                        }
                                    }).collect_view()}
                                </div>
                            }.into_any()
                        }
                    } else {
                        view! {
                            <div class="py-8 text-center">
                                <Text tone=TextTone::Muted>"-"</Text>
                            </div>
                        }.into_any()
                    }
                }}
            </CardBody>
        </Card>
    }
}

#[cfg(not(feature = "hydrate"))]
fn clicks_timeline_section(
    _clicks_by_date: RwSignal<Option<ClicksByDate>>,
    _loading: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <Card>
            <CardHeader>
                <Text tone=TextTone::Muted class="text-xs uppercase tracking-wide">
                    "Clics par jour"
                </Text>
            </CardHeader>
            <CardBody class="pt-4">
                <div class="py-8 text-center">
                    <Text tone=TextTone::Muted>"Chargement..."</Text>
                </div>
            </CardBody>
        </Card>
    }
}

// ===== Geographic Section =====

#[cfg(feature = "hydrate")]
fn geographic_section(
    geo_data: RwSignal<Option<GeographicBreakdown>>,
    loading: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <Card>
            <CardHeader>
                <Text tone=TextTone::Muted class="text-xs uppercase tracking-wide">
                    "Geographie"
                </Text>
            </CardHeader>
            <CardBody class="pt-4">
                {move || {
                    if loading.get() {
                        view! {
                            <div class="py-8 text-center">
                                <Text tone=TextTone::Muted>"Chargement..."</Text>
                            </div>
                        }.into_any()
                    } else if let Some(data) = geo_data.get() {
                        view! {
                            <div class="grid gap-6 md:grid-cols-2">
                                <div>
                                    <Text class="text-sm font-medium mb-3">"Par pays"</Text>
                                    <div class="space-y-2">
                                        {if data.countries.is_empty() {
                                            view! { <Text tone=TextTone::Muted class="text-sm">"Aucune donnee"</Text> }.into_any()
                                        } else {
                                            data.countries.into_iter().map(|country| {
                                                view! {
                                                    <div class="flex items-center justify-between py-1">
                                                        <Text class="text-sm">{country.name.clone()}</Text>
                                                        <div class="flex items-center gap-2">
                                                            <Text tone=TextTone::Subtle class="text-xs">
                                                                {format!("{:.1}%", country.percentage)}
                                                            </Text>
                                                            <Badge variant=BadgeVariant::Outline class="text-xs">
                                                                {country.clicks.to_string()}
                                                            </Badge>
                                                        </div>
                                                    </div>
                                                }
                                            }).collect_view().into_any()
                                        }}
                                    </div>
                                </div>
                                <div>
                                    <Text class="text-sm font-medium mb-3">"Par ville"</Text>
                                    <div class="space-y-2">
                                        {if data.cities.is_empty() {
                                            view! { <Text tone=TextTone::Muted class="text-sm">"Aucune donnee"</Text> }.into_any()
                                        } else {
                                            data.cities.into_iter().map(|city| {
                                                view! {
                                                    <div class="flex items-center justify-between py-1">
                                                        <Text class="text-sm">{city.name.clone()}</Text>
                                                        <div class="flex items-center gap-2">
                                                            <Text tone=TextTone::Subtle class="text-xs">
                                                                {format!("{:.1}%", city.percentage)}
                                                            </Text>
                                                            <Badge variant=BadgeVariant::Outline class="text-xs">
                                                                {city.clicks.to_string()}
                                                            </Badge>
                                                        </div>
                                                    </div>
                                                }
                                            }).collect_view().into_any()
                                        }}
                                    </div>
                                </div>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="py-8 text-center">
                                <Text tone=TextTone::Muted>"-"</Text>
                            </div>
                        }.into_any()
                    }
                }}
            </CardBody>
        </Card>
    }
}

#[cfg(not(feature = "hydrate"))]
fn geographic_section(
    _geo_data: RwSignal<Option<GeographicBreakdown>>,
    _loading: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <Card>
            <CardHeader>
                <Text tone=TextTone::Muted class="text-xs uppercase tracking-wide">
                    "Geographie"
                </Text>
            </CardHeader>
            <CardBody class="pt-4">
                <div class="py-8 text-center">
                    <Text tone=TextTone::Muted>"Chargement..."</Text>
                </div>
            </CardBody>
        </Card>
    }
}

// ===== Referrers Section =====

#[cfg(feature = "hydrate")]
fn referrers_section(
    referrer_data: RwSignal<Option<ReferrerBreakdown>>,
    loading: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <Card>
            <CardHeader>
                <Text tone=TextTone::Muted class="text-xs uppercase tracking-wide">
                    "Sources de trafic"
                </Text>
            </CardHeader>
            <CardBody class="pt-4">
                {move || {
                    if loading.get() {
                        view! {
                            <div class="py-8 text-center">
                                <Text tone=TextTone::Muted>"Chargement..."</Text>
                            </div>
                        }.into_any()
                    } else if let Some(data) = referrer_data.get() {
                        if data.referrers.is_empty() {
                            view! {
                                <div class="py-8 text-center">
                                    <Text tone=TextTone::Muted>"Aucune donnee disponible"</Text>
                                </div>
                            }.into_any()
                        } else {
                            let max_clicks = data.referrers.iter().map(|r| r.clicks).max().unwrap_or(1);
                            view! {
                                <div class="space-y-3">
                                    {data.referrers.into_iter().map(|referrer| {
                                        let percentage = (referrer.clicks as f64 / max_clicks as f64) * 100.0;
                                        view! {
                                            <div>
                                                <div class="flex items-center justify-between mb-1">
                                                    <Text class="text-sm font-medium">{referrer.source.clone()}</Text>
                                                    <div class="flex items-center gap-2">
                                                        <Text tone=TextTone::Subtle class="text-xs">
                                                            {format!("{:.1}%", referrer.percentage)}
                                                        </Text>
                                                        <Badge variant=BadgeVariant::Outline class="text-xs">
                                                            {referrer.clicks.to_string()}
                                                        </Badge>
                                                    </div>
                                                </div>
                                                <ProgressBar percentage=percentage color="bg-brand/70".to_string() />
                                            </div>
                                        }
                                    }).collect_view()}
                                </div>
                            }.into_any()
                        }
                    } else {
                        view! {
                            <div class="py-8 text-center">
                                <Text tone=TextTone::Muted>"-"</Text>
                            </div>
                        }.into_any()
                    }
                }}
            </CardBody>
        </Card>
    }
}

#[cfg(not(feature = "hydrate"))]
fn referrers_section(
    _referrer_data: RwSignal<Option<ReferrerBreakdown>>,
    _loading: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <Card>
            <CardHeader>
                <Text tone=TextTone::Muted class="text-xs uppercase tracking-wide">
                    "Sources de trafic"
                </Text>
            </CardHeader>
            <CardBody class="pt-4">
                <div class="py-8 text-center">
                    <Text tone=TextTone::Muted>"Chargement..."</Text>
                </div>
            </CardBody>
        </Card>
    }
}

// ===== Devices Section =====

#[cfg(feature = "hydrate")]
fn devices_section(
    device_data: RwSignal<Option<DeviceBreakdown>>,
    loading: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <Card>
            <CardHeader>
                <Text tone=TextTone::Muted class="text-xs uppercase tracking-wide">
                    "Appareils et navigateurs"
                </Text>
            </CardHeader>
            <CardBody class="pt-4">
                {move || {
                    if loading.get() {
                        view! {
                            <div class="py-8 text-center">
                                <Text tone=TextTone::Muted>"Chargement..."</Text>
                            </div>
                        }.into_any()
                    } else if let Some(data) = device_data.get() {
                        view! {
                            <div class="grid gap-6 md:grid-cols-2">
                                <div>
                                    <Text class="text-sm font-medium mb-3">"Navigateurs"</Text>
                                    <div class="space-y-2">
                                        {if data.browsers.is_empty() {
                                            view! { <Text tone=TextTone::Muted class="text-sm">"Aucune donnee"</Text> }.into_any()
                                        } else {
                                            data.browsers.into_iter().map(|browser| {
                                                view! {
                                                    <div class="flex items-center justify-between py-1">
                                                        <Text class="text-sm">{browser.name.clone()}</Text>
                                                        <div class="flex items-center gap-2">
                                                            <Text tone=TextTone::Subtle class="text-xs">
                                                                {format!("{:.1}%", browser.percentage)}
                                                            </Text>
                                                            <Badge variant=BadgeVariant::Outline class="text-xs">
                                                                {browser.clicks.to_string()}
                                                            </Badge>
                                                        </div>
                                                    </div>
                                                }
                                            }).collect_view().into_any()
                                        }}
                                    </div>
                                </div>
                                <div>
                                    <Text class="text-sm font-medium mb-3">"Appareils"</Text>
                                    <div class="space-y-2">
                                        {if data.devices.is_empty() {
                                            view! { <Text tone=TextTone::Muted class="text-sm">"Aucune donnee"</Text> }.into_any()
                                        } else {
                                            data.devices.into_iter().map(|device| {
                                                view! {
                                                    <div class="flex items-center justify-between py-1">
                                                        <Text class="text-sm">{device.name.clone()}</Text>
                                                        <div class="flex items-center gap-2">
                                                            <Text tone=TextTone::Subtle class="text-xs">
                                                                {format!("{:.1}%", device.percentage)}
                                                            </Text>
                                                            <Badge variant=BadgeVariant::Outline class="text-xs">
                                                                {device.clicks.to_string()}
                                                            </Badge>
                                                        </div>
                                                    </div>
                                                }
                                            }).collect_view().into_any()
                                        }}
                                    </div>
                                </div>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="py-8 text-center">
                                <Text tone=TextTone::Muted>"-"</Text>
                            </div>
                        }.into_any()
                    }
                }}
            </CardBody>
        </Card>
    }
}

#[cfg(not(feature = "hydrate"))]
fn devices_section(
    _device_data: RwSignal<Option<DeviceBreakdown>>,
    _loading: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <Card>
            <CardHeader>
                <Text tone=TextTone::Muted class="text-xs uppercase tracking-wide">
                    "Appareils et navigateurs"
                </Text>
            </CardHeader>
            <CardBody class="pt-4">
                <div class="py-8 text-center">
                    <Text tone=TextTone::Muted>"Chargement..."</Text>
                </div>
            </CardBody>
        </Card>
    }
}

// ===== Link Detail Modal =====

#[cfg(feature = "hydrate")]
fn link_detail_modal(
    show_modal: RwSignal<bool>,
    link_analytics: RwSignal<Option<LinkDetailedAnalytics>>,
    loading: RwSignal<bool>,
    error: RwSignal<Option<String>>,
) -> impl IntoView {
    view! {
        {move || {
            if show_modal.get() {
                Some(view! {
                    <div
                        class="fixed inset-0 z-50 flex items-start justify-center overflow-y-auto bg-black/60 backdrop-blur py-8"
                        on:click=move |_| {
                            show_modal.set(false);
                            link_analytics.set(None);
                            error.set(None);
                        }
                    >
                        <div
                            class="mx-4 w-full max-w-4xl rounded-2xl border border-border/60 bg-background p-6 shadow-xl my-auto"
                            on:click=move |e| {
                                e.stop_propagation();
                            }
                        >
                            <div class="mb-6 flex items-center justify-between">
                                <Heading level=HeadingLevel::H2 class="text-lg">
                                    "Analytiques du lien"
                                </Heading>
                                <button
                                    class="text-foreground/60 hover:text-foreground transition"
                                    on:click=move |_| {
                                        show_modal.set(false);
                                        link_analytics.set(None);
                                        error.set(None);
                                    }
                                >
                                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                        <line x1="18" y1="6" x2="6" y2="18"></line>
                                        <line x1="6" y1="6" x2="18" y2="18"></line>
                                    </svg>
                                </button>
                            </div>

                            {move || {
                                if loading.get() {
                                    view! {
                                        <div class="py-12 text-center">
                                            <Text tone=TextTone::Muted>"Chargement des analytiques..."</Text>
                                        </div>
                                    }.into_any()
                                } else if let Some(err) = error.get() {
                                    view! {
                                        <div class="rounded-xl border border-danger/50 bg-danger/10 px-4 py-3 text-sm text-danger">
                                            {format!("Erreur: {}", err)}
                                        </div>
                                    }.into_any()
                                } else if let Some(data) = link_analytics.get() {
                                    view! {
                                        <div class="space-y-6">
                                            // Link info header
                                            <div class="rounded-xl border border-border/60 bg-surface/60 p-4">
                                                <div class="flex items-center justify-between">
                                                    <div>
                                                        <Heading level=HeadingLevel::H3 class="text-base">
                                                            {data.title.clone().unwrap_or_else(|| data.short_code.clone())}
                                                        </Heading>
                                                        <Text tone=TextTone::Subtle class="text-xs truncate max-w-md">
                                                            {data.original_url.clone()}
                                                        </Text>
                                                    </div>
                                                    <div class="text-right">
                                                        <Heading level=HeadingLevel::H2 class="text-2xl text-brand">
                                                            {data.total_clicks.to_string()}
                                                        </Heading>
                                                        <Text tone=TextTone::Muted class="text-xs">"total clics"</Text>
                                                    </div>
                                                </div>
                                            </div>

                                            // Clicks over time
                                            <div>
                                                <Text class="text-sm font-medium mb-3">"Clics par jour"</Text>
                                                {if data.clicks_by_date.is_empty() {
                                                    view! { <Text tone=TextTone::Muted class="text-sm">"Aucune donnee"</Text> }.into_any()
                                                } else {
                                                    let max_clicks = data.clicks_by_date.iter().map(|d| d.clicks).max().unwrap_or(1);
                                                    view! {
                                                        <div class="space-y-2 max-h-[200px] overflow-y-auto">
                                                            {data.clicks_by_date.clone().into_iter().rev().take(10).map(|day| {
                                                                let percentage = (day.clicks as f64 / max_clicks as f64) * 100.0;
                                                                view! {
                                                                    <div class="flex items-center gap-4">
                                                                        <Text tone=TextTone::Subtle class="text-xs w-24 text-right font-mono">
                                                                            {day.date.clone()}
                                                                        </Text>
                                                                        <div class="flex-1">
                                                                            <ProgressBar percentage=percentage />
                                                                        </div>
                                                                        <Text class="text-sm w-12 text-right font-medium">
                                                                            {day.clicks.to_string()}
                                                                        </Text>
                                                                    </div>
                                                                }
                                                            }).collect_view()}
                                                        </div>
                                                    }.into_any()
                                                }}
                                            </div>

                                            // Grid for other stats
                                            <div class="grid gap-6 md:grid-cols-2">
                                                // Geographic
                                                <div class="rounded-xl border border-border/40 p-4">
                                                    <Text class="text-sm font-medium mb-3">"Pays"</Text>
                                                    <div class="space-y-2">
                                                        {if data.geographic.countries.is_empty() {
                                                            view! { <Text tone=TextTone::Muted class="text-sm">"Aucune donnee"</Text> }.into_any()
                                                        } else {
                                                            data.geographic.countries.clone().into_iter().take(5).map(|country| {
                                                                view! {
                                                                    <div class="flex items-center justify-between py-1">
                                                                        <Text class="text-sm">{country.name.clone()}</Text>
                                                                        <Badge variant=BadgeVariant::Outline class="text-xs">
                                                                            {format!("{} ({:.0}%)", country.clicks, country.percentage)}
                                                                        </Badge>
                                                                    </div>
                                                                }
                                                            }).collect_view().into_any()
                                                        }}
                                                    </div>
                                                </div>

                                                // Referrers
                                                <div class="rounded-xl border border-border/40 p-4">
                                                    <Text class="text-sm font-medium mb-3">"Sources"</Text>
                                                    <div class="space-y-2">
                                                        {if data.referrers.referrers.is_empty() {
                                                            view! { <Text tone=TextTone::Muted class="text-sm">"Aucune donnee"</Text> }.into_any()
                                                        } else {
                                                            data.referrers.referrers.clone().into_iter().take(5).map(|referrer| {
                                                                view! {
                                                                    <div class="flex items-center justify-between py-1">
                                                                        <Text class="text-sm">{referrer.source.clone()}</Text>
                                                                        <Badge variant=BadgeVariant::Outline class="text-xs">
                                                                            {format!("{} ({:.0}%)", referrer.clicks, referrer.percentage)}
                                                                        </Badge>
                                                                    </div>
                                                                }
                                                            }).collect_view().into_any()
                                                        }}
                                                    </div>
                                                </div>

                                                // Browsers
                                                <div class="rounded-xl border border-border/40 p-4">
                                                    <Text class="text-sm font-medium mb-3">"Navigateurs"</Text>
                                                    <div class="space-y-2">
                                                        {if data.devices.browsers.is_empty() {
                                                            view! { <Text tone=TextTone::Muted class="text-sm">"Aucune donnee"</Text> }.into_any()
                                                        } else {
                                                            data.devices.browsers.clone().into_iter().take(5).map(|browser| {
                                                                view! {
                                                                    <div class="flex items-center justify-between py-1">
                                                                        <Text class="text-sm">{browser.name.clone()}</Text>
                                                                        <Badge variant=BadgeVariant::Outline class="text-xs">
                                                                            {format!("{} ({:.0}%)", browser.clicks, browser.percentage)}
                                                                        </Badge>
                                                                    </div>
                                                                }
                                                            }).collect_view().into_any()
                                                        }}
                                                    </div>
                                                </div>

                                                // Devices
                                                <div class="rounded-xl border border-border/40 p-4">
                                                    <Text class="text-sm font-medium mb-3">"Appareils"</Text>
                                                    <div class="space-y-2">
                                                        {if data.devices.devices.is_empty() {
                                                            view! { <Text tone=TextTone::Muted class="text-sm">"Aucune donnee"</Text> }.into_any()
                                                        } else {
                                                            data.devices.devices.clone().into_iter().take(5).map(|device| {
                                                                view! {
                                                                    <div class="flex items-center justify-between py-1">
                                                                        <Text class="text-sm">{device.name.clone()}</Text>
                                                                        <Badge variant=BadgeVariant::Outline class="text-xs">
                                                                            {format!("{} ({:.0}%)", device.clicks, device.percentage)}
                                                                        </Badge>
                                                                    </div>
                                                                }
                                                            }).collect_view().into_any()
                                                        }}
                                                    </div>
                                                </div>
                                            </div>

                                            <div class="flex justify-end">
                                                <Button
                                                    variant=ButtonVariant::Secondary
                                                    size=ButtonSize::Md
                                                    on:click=move |_| {
                                                        show_modal.set(false);
                                                        link_analytics.set(None);
                                                    }
                                                >
                                                    "Fermer"
                                                </Button>
                                            </div>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div class="py-12 text-center">
                                            <Text tone=TextTone::Muted>"Aucune donnee"</Text>
                                        </div>
                                    }.into_any()
                                }
                            }}
                        </div>
                    </div>
                })
            } else {
                None
            }
        }}
    }
}

#[cfg(not(feature = "hydrate"))]
fn link_detail_modal(
    _show_modal: RwSignal<bool>,
    _link_analytics: RwSignal<Option<LinkDetailedAnalytics>>,
    _loading: RwSignal<bool>,
    _error: RwSignal<Option<String>>,
) -> impl IntoView {
    view! {}
}

// ===== Date Filter Component =====

#[component]
fn DateFilters(
    start_date: RwSignal<String>,
    end_date: RwSignal<String>,
    on_apply: Callback<()>,
) -> impl IntoView {
    view! {
        <div class="flex flex-wrap items-end gap-4">
            <FormControl label="Date debut">
                <InputField
                    id="start-date"
                    name="start_date"
                    input_type="date"
                    value=start_date
                    on_input=Callback::new(move |ev| start_date.set(event_target_value(&ev)))
                />
            </FormControl>
            <FormControl label="Date fin">
                <InputField
                    id="end-date"
                    name="end_date"
                    input_type="date"
                    value=end_date
                    on_input=Callback::new(move |ev| end_date.set(event_target_value(&ev)))
                />
            </FormControl>
            <Button
                variant=ButtonVariant::Secondary
                size=ButtonSize::Md
                on:click=move |_| on_apply.run(())
            >
                "Appliquer"
            </Button>
        </div>
    }
}

// ===== Main Analytics Page =====

#[component]
pub fn AnalyticsPage() -> impl IntoView {
    let workspace_store = use_workspace_store();

    // Overview data
    let overview = RwSignal::new(Option::<AnalyticsOverview>::None);
    let overview_loading = RwSignal::new(false);
    let overview_error = RwSignal::new(Option::<String>::None);

    // Clicks by date
    let clicks_by_date = RwSignal::new(Option::<ClicksByDate>::None);
    let clicks_loading = RwSignal::new(false);

    // Geographic data
    let geo_data = RwSignal::new(Option::<GeographicBreakdown>::None);
    let geo_loading = RwSignal::new(false);

    // Referrer data
    let referrer_data = RwSignal::new(Option::<ReferrerBreakdown>::None);
    let referrer_loading = RwSignal::new(false);

    // Device data
    let device_data = RwSignal::new(Option::<DeviceBreakdown>::None);
    let device_loading = RwSignal::new(false);

    // Link detail modal
    let show_link_modal = RwSignal::new(false);
    let link_analytics = RwSignal::new(Option::<LinkDetailedAnalytics>::None);
    let link_loading = RwSignal::new(false);
    let link_error = RwSignal::new(Option::<String>::None);

    // Date filters
    let start_date = RwSignal::new(String::new());
    let end_date = RwSignal::new(String::new());

    #[cfg(feature = "hydrate")]
    let auth_store = use_auth_store();
    #[cfg(feature = "hydrate")]
    let selected_workspace = workspace_store.selected_workspace();

    // Load data effect
    #[cfg(feature = "hydrate")]
    {
        let selected_workspace_effect = selected_workspace.clone();
        let auth_store_effect = auth_store.clone();

        Effect::new(move |_| {
            if let Some(ws) = selected_workspace_effect.get() {
                if let Some(token) = auth_store_effect.access_token() {
                    let workspace_id = ws.id.clone();
                    let token_clone = token.clone();

                    // Load overview
                    overview_loading.set(true);
                    overview_error.set(None);
                    let ws_id = workspace_id.clone();
                    let tk = token_clone.clone();
                    spawn_local(async move {
                        match fetch_analytics_overview(&ws_id, &tk, None, None).await {
                            Ok(data) => {
                                overview.set(Some(data));
                                overview_loading.set(false);
                            }
                            Err(err) => {
                                overview_error.set(Some(err));
                                overview_loading.set(false);
                            }
                        }
                    });

                    // Load clicks by date
                    clicks_loading.set(true);
                    let ws_id = workspace_id.clone();
                    let tk = token_clone.clone();
                    spawn_local(async move {
                        if let Ok(data) = fetch_clicks_by_date(&ws_id, &tk, None, None).await {
                            clicks_by_date.set(Some(data));
                        }
                        clicks_loading.set(false);
                    });

                    // Load geographic data
                    geo_loading.set(true);
                    let ws_id = workspace_id.clone();
                    let tk = token_clone.clone();
                    spawn_local(async move {
                        if let Ok(data) = fetch_geographic_breakdown(&ws_id, &tk).await {
                            geo_data.set(Some(data));
                        }
                        geo_loading.set(false);
                    });

                    // Load referrer data
                    referrer_loading.set(true);
                    let ws_id = workspace_id.clone();
                    let tk = token_clone.clone();
                    spawn_local(async move {
                        if let Ok(data) = fetch_referrer_breakdown(&ws_id, &tk).await {
                            referrer_data.set(Some(data));
                        }
                        referrer_loading.set(false);
                    });

                    // Load device data
                    device_loading.set(true);
                    let ws_id = workspace_id.clone();
                    let tk = token_clone.clone();
                    spawn_local(async move {
                        if let Ok(data) = fetch_device_breakdown(&ws_id, &tk).await {
                            device_data.set(Some(data));
                        }
                        device_loading.set(false);
                    });
                }
            } else {
                overview.set(None);
                clicks_by_date.set(None);
                geo_data.set(None);
                referrer_data.set(None);
                device_data.set(None);
            }
        });
    }

    // Handler for applying date filter
    #[cfg(feature = "hydrate")]
    let on_apply_filter = {
        let auth_store = auth_store.clone();
        let selected_workspace = selected_workspace.clone();
        Callback::new(move |_| {
            if let Some(ws) = selected_workspace.get() {
                if let Some(token) = auth_store.access_token() {
                    let workspace_id = ws.id.clone();
                    let start = if start_date.get().is_empty() {
                        None
                    } else {
                        Some(start_date.get())
                    };
                    let end = if end_date.get().is_empty() {
                        None
                    } else {
                        Some(end_date.get())
                    };

                    // Reload overview
                    overview_loading.set(true);
                    let ws_id = workspace_id.clone();
                    let tk = token.clone();
                    let start_clone = start.clone();
                    let end_clone = end.clone();
                    spawn_local(async move {
                        match fetch_analytics_overview(
                            &ws_id,
                            &tk,
                            start_clone.as_deref(),
                            end_clone.as_deref(),
                        )
                        .await
                        {
                            Ok(data) => {
                                overview.set(Some(data));
                                overview_loading.set(false);
                            }
                            Err(err) => {
                                overview_error.set(Some(err));
                                overview_loading.set(false);
                            }
                        }
                    });

                    // Reload clicks by date
                    clicks_loading.set(true);
                    let ws_id = workspace_id.clone();
                    let tk = token.clone();
                    let start_clone = start.clone();
                    let end_clone = end.clone();
                    spawn_local(async move {
                        if let Ok(data) = fetch_clicks_by_date(
                            &ws_id,
                            &tk,
                            start_clone.as_deref(),
                            end_clone.as_deref(),
                        )
                        .await
                        {
                            clicks_by_date.set(Some(data));
                        }
                        clicks_loading.set(false);
                    });
                }
            }
        })
    };

    #[cfg(not(feature = "hydrate"))]
    let on_apply_filter = Callback::new(|_| {});

    // Handler for selecting a link
    #[cfg(feature = "hydrate")]
    let on_select_link = {
        let auth_store = auth_store.clone();
        Callback::new(move |link_id: String| {
            if let Some(token) = auth_store.access_token() {
                show_link_modal.set(true);
                link_loading.set(true);
                link_error.set(None);
                link_analytics.set(None);

                let tk = token.clone();
                spawn_local(async move {
                    match fetch_link_analytics(&link_id, &tk, None, None).await {
                        Ok(data) => {
                            link_analytics.set(Some(data));
                            link_loading.set(false);
                        }
                        Err(err) => {
                            link_error.set(Some(err));
                            link_loading.set(false);
                        }
                    }
                });
            }
        })
    };

    #[cfg(not(feature = "hydrate"))]
    let on_select_link = Callback::new(|_: String| {});

    #[cfg(feature = "hydrate")]
    let selected_workspace_view = selected_workspace.clone();
    #[cfg(not(feature = "hydrate"))]
    let selected_workspace_view = Signal::derive(|| None::<WorkspaceSummary>);

    let has_workspace = Signal::derive(move || selected_workspace_view.get().is_some());

    view! {
        <DashboardLayout>
            <div class="flex flex-col gap-8">
                // Header
                <div class="flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
                    <div class="flex flex-col gap-2">
                        <Heading level=HeadingLevel::H1>
                            "Analytics"
                        </Heading>
                        <Text tone=TextTone::Muted>
                            "Suivez les performances de vos liens raccourcis"
                        </Text>
                    </div>
                </div>

                // No workspace selected
                <Show
                    when=move || !has_workspace.get()
                    fallback=move || ()
                >
                    <Card>
                        <CardBody class="pt-6">
                            <div class="flex flex-col items-center justify-center gap-6 py-12 text-center">
                                <div class="flex flex-col gap-2">
                                    <Heading level=HeadingLevel::H2 class="text-xl">
                                        "Aucun workspace selectionne"
                                    </Heading>
                                    <Text tone=TextTone::Muted class="text-sm">
                                        "Selectionnez un workspace pour voir les analytiques"
                                    </Text>
                                </div>
                            </div>
                        </CardBody>
                    </Card>
                </Show>

                // Analytics content
                <Show
                    when=move || has_workspace.get()
                    fallback=move || ()
                >
                    // Date filters
                    <Card>
                        <CardBody class="pt-6">
                            <DateFilters
                                start_date=start_date
                                end_date=end_date
                                on_apply=on_apply_filter.clone()
                            />
                        </CardBody>
                    </Card>

                    // Overview stats
                    {overview_section(overview, overview_loading, overview_error)}

                    // Main content grid
                    <div class="grid gap-6 lg:grid-cols-2">
                        // Top links
                        {top_links_section(overview, on_select_link.clone())}

                        // Clicks timeline
                        {clicks_timeline_section(clicks_by_date, clicks_loading)}
                    </div>

                    // Geographic breakdown
                    {geographic_section(geo_data, geo_loading)}

                    // Referrers and Devices
                    <div class="grid gap-6 lg:grid-cols-2">
                        {referrers_section(referrer_data, referrer_loading)}
                        {devices_section(device_data, device_loading)}
                    </div>
                </Show>
            </div>

            // Link detail modal
            {link_detail_modal(show_link_modal, link_analytics, link_loading, link_error)}
        </DashboardLayout>
    }
}
