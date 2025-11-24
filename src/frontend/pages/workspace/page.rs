use crate::frontend::{
    design_system::{
        Badge, BadgeVariant, Button, ButtonVariant, Card, CardBody, CardHeader,
        Heading, HeadingLevel, Text, TextTone,
    },
    layouts::DashboardLayout,
    state::use_workspace_store,
};
use crate::frontend::pages::workspace::{LinkSummary, WorkspaceStats};
use leptos::prelude::*;
#[cfg(feature = "hydrate")]
use leptos::task::spawn_local;
#[cfg(feature = "hydrate")]
use crate::frontend::state::use_auth_store;
#[cfg(feature = "hydrate")]
use crate::frontend::pages::workspace::client::{fetch_workspace_stats, fetch_recent_links};

#[component]
pub fn WorkspacePage() -> impl IntoView {
    let workspace_store = use_workspace_store();
    let selected_workspace = workspace_store.selected_workspace();
    let workspace_label = Signal::derive(move || {
        selected_workspace
            .get()
            .map(|ws| ws.name.clone())
            .unwrap_or_else(|| "Aucun workspace".to_string())
    });

    let stats = RwSignal::new(Option::<WorkspaceStats>::None);
    let stats_loading = RwSignal::new(false);
    let stats_error = RwSignal::new(Option::<String>::None);

    let recent_links = RwSignal::new(Vec::<LinkSummary>::new());
    let links_loading = RwSignal::new(false);
    let links_error = RwSignal::new(Option::<String>::None);

    #[cfg(feature = "hydrate")]
    {
        let auth_store = use_auth_store();
        let selected_workspace_for_effect = selected_workspace.clone();
        let stats_for_effect = stats.clone();
        let stats_loading_for_effect = stats_loading.clone();
        let stats_error_for_effect = stats_error.clone();
        
        let recent_links_for_effect = recent_links.clone();
        let links_loading_for_effect = links_loading.clone();
        let links_error_for_effect = links_error.clone();
        
        Effect::new(move |_| {
            if let Some(ws) = selected_workspace_for_effect.get() {
                // Charger les stats
                stats_loading_for_effect.set(true);
                stats_error_for_effect.set(None);
                
                let workspace_id = ws.id.clone();
                let auth_store_clone = auth_store.clone();
                let stats_clone = stats_for_effect.clone();
                let loading_clone = stats_loading_for_effect.clone();
                let error_clone = stats_error_for_effect.clone();
                
                spawn_local(async move {
                    if let Some(token) = auth_store_clone.access_token() {
                        match fetch_workspace_stats(&workspace_id, &token).await {
                            Ok(ws_stats) => {
                                loading_clone.set(false);
                                stats_clone.set(Some(ws_stats));
                            }
                            Err(err) => {
                                loading_clone.set(false);
                                error_clone.set(Some(err));
                            }
                        }
                    }
                });

                // Charger les liens récents
                links_loading_for_effect.set(true);
                links_error_for_effect.set(None);
                
                let workspace_id_links = ws.id.clone();
                let auth_store_links = auth_store.clone();
                let links_clone = recent_links_for_effect.clone();
                let links_loading_clone = links_loading_for_effect.clone();
                let links_error_clone = links_error_for_effect.clone();
                
                spawn_local(async move {
                    if let Some(token) = auth_store_links.access_token() {
                        match fetch_recent_links(&workspace_id_links, &token, 5).await {
                            Ok(links) => {
                                links_loading_clone.set(false);
                                links_clone.set(links);
                            }
                            Err(err) => {
                                links_loading_clone.set(false);
                                links_error_clone.set(Some(err));
                            }
                        }
                    }
                });
            } else {
                stats_for_effect.set(None);
                recent_links_for_effect.set(Vec::new());
            }
        });
    }

    view! {
        <DashboardLayout>
            <section class="grid gap-6">
                <div class="flex items-center justify-between">
                    <Heading level=HeadingLevel::H1>
                        {move || workspace_label.get()}
                    </Heading>
                    <Button variant=ButtonVariant::Primary>
                        "Créer un lien"
                    </Button>
                </div>

                <div class="grid gap-6 md:grid-cols-2">
                    <Card>
                        <CardHeader class="justify-center min-h-[120px] pb-6">
                            <Text tone=TextTone::Muted class="text-xs uppercase tracking-wide">
                                "Total de liens"
                            </Text>
                            <Heading level=HeadingLevel::H2>
                                {move || {
                                    stats.get()
                                        .map(|s| s.total_links.to_string())
                                        .unwrap_or_else(|| {
                                            if stats_loading.get() {
                                                "Chargement...".to_string()
                                            } else {
                                                "-".to_string()
                                            }
                                        })
                                }}
                            </Heading>
                        </CardHeader>
                    </Card>

                    <Card>
                        <CardHeader class="justify-center min-h-[120px] pb-6">
                            <Text tone=TextTone::Muted class="text-xs uppercase tracking-wide">
                                "Total de clics"
                            </Text>
                            <Heading level=HeadingLevel::H2>
                                {move || {
                                    stats.get()
                                        .map(|s| s.total_clicks.to_string())
                                        .unwrap_or_else(|| {
                                            if stats_loading.get() {
                                                "Chargement...".to_string()
                                            } else {
                                                "-".to_string()
                                            }
                                        })
                                }}
                            </Heading>
                        </CardHeader>
                    </Card>
                </div>

                {move || {
                    stats_error.get().map(|err| {
                        view! {
                            <div class="rounded-xl border border-danger/50 bg-danger/10 px-4 py-3 text-sm text-danger">
                                {format!("Erreur: {}", err)}
                            </div>
                        }
                        .into_any()
                    })
                }}

                <Card>
                    <CardHeader>
                        <Text tone=TextTone::Muted class="text-xs uppercase tracking-wide">
                            "Liens récents"
                        </Text>
                    </CardHeader>
                    <CardBody>
                        {move || {
                            if links_loading.get() {
                                view! { <Text>"Chargement des liens..."</Text> }.into_any()
                            } else if let Some(err) = links_error.get() {
                                view! { <Text tone=TextTone::Danger>{format!("Erreur: {}", err)}</Text> }.into_any()
                            } else if recent_links.get().is_empty() {
                                view! { <Text>"Aucun lien récent."</Text> }.into_any()
                            } else {
                                recent_links.get().into_iter().map(|link| {
                                    view! {
                                        <div class="flex items-center justify-between py-2">
                                            <div>
                                                <Text class="font-medium">{link.title.unwrap_or_else(|| link.original_url.clone())}</Text>
                                                <Text tone=TextTone::Subtle class="font-mono text-xs">{link.short_url}</Text>
                                            </div>
                                            <Badge variant={if link.is_active { BadgeVariant::Subtle } else { BadgeVariant::Outline }}>
                                                {if link.is_active { "Actif" } else { "Inactif" }}
                                            </Badge>
                                        </div>
                                    }
                                }).collect_view().into_any()
                            }
                        }}
                    </CardBody>
                </Card>
            </section>
        </DashboardLayout>
    }
}

