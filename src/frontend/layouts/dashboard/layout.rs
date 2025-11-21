use super::{footer::DashboardFooter, navbar::DashboardNavbar, settings_modal::SettingsModal};
use crate::frontend::state::use_auth_store;
use leptos::prelude::*;
#[cfg(feature = "hydrate")]
use leptos_router::hooks::{use_location, use_navigate};
#[cfg(feature = "hydrate")]
use leptos_router::NavigateOptions;

#[component]
pub fn DashboardLayout(children: Children) -> impl IntoView {
    let auth_store = use_auth_store();
    let show_settings_modal = RwSignal::new(false);

    let open_settings = {
        let show_settings_modal = show_settings_modal;
        Callback::new(move |_| {
            show_settings_modal.set(true);
        })
    };

    let is_authenticated = auth_store.is_authenticated();
    let initializing = auth_store.initializing();

    #[cfg(feature = "hydrate")]
    {
        let auth = auth_store.clone();
        let navigate = use_navigate();
        let location = use_location();
        leptos::prelude::Effect::new(move |_| {
            if !auth.initializing().get() && !auth.is_authenticated().get() {
                let path = location.pathname.get();
                if path.starts_with("/login") || path.starts_with("/register") {
                    return;
                }
                let search = location.search.get();
                let redirect = if path.is_empty() || path == "/" {
                    "/login".to_string()
                } else {
                    let mut target = format!("/login?redirect={}", path);
                    if !search.is_empty() {
                        target.push_str(&search);
                    }
                    target
                };
                let _ = navigate(
                    &redirect,
                    NavigateOptions {
                        replace: true,
                        ..Default::default()
                    },
                );
            }
        });
    }

    let main_view = move || {
        if initializing.get() {
            view! {
                <div class="flex min-h-screen items-center justify-center bg-background text-sm text-foreground/70">
                    "Chargement du tableau de bord…"
                </div>
            }
            .into_any()
        } else if is_authenticated.get() {
            view! {
                <div class="flex min-h-screen flex-col bg-background text-foreground">
                    <DashboardNavbar on_open_settings=open_settings.clone()/>
                    <main class="flex-1 bg-surface/40">
                        <div class="mx-auto w-full max-w-6xl px-4 py-8 sm:px-6 lg:px-12">
                            {children()}
                        </div>
                    </main>
                    <DashboardFooter/>
                </div>
            }
            .into_any()
        } else {
            view! {
                <div class="flex min-h-screen items-center justify-center bg-background text-sm text-foreground/70">
                    "Redirection vers la page de connexion…"
                </div>
            }
            .into_any()
        }
    };

    view! {
        {main_view()}
        <SettingsModal open=show_settings_modal/>
    }
}
