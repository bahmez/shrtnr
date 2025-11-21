use super::{
    footer::DashboardFooter, navbar::DashboardNavbar, settings_modal::SettingsModal,
    workspace_modal::WorkspaceModal,
};
use crate::frontend::state::{provide_workspace_store, use_auth_store};
use leptos::prelude::*;
#[cfg(feature = "hydrate")]
use leptos_router::hooks::{use_location, use_navigate};
#[cfg(feature = "hydrate")]
use leptos_router::NavigateOptions;

#[component]
pub fn DashboardLayout(children: ChildrenFn) -> impl IntoView {
    let auth_store = use_auth_store();
    let show_settings_modal = RwSignal::new(false);
    let show_workspace_modal = RwSignal::new(false);
    let workspace_store = provide_workspace_store(auth_store.clone());

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

        {
            let workspace_store = workspace_store.clone();
            let user_signal = auth_store.user();
            Effect::new(move |_| {
                if let Some(user) = user_signal.get() {
                    workspace_store.load_for_owner(user.id.clone());
                } else {
                    workspace_store.clear();
                }
            });
        }
    }

    #[cfg(not(feature = "hydrate"))]
    {
        workspace_store.clear();
    }

    let children_store = StoredValue::new(children.clone());

    view! {
        <Show
            when=move || !initializing.get()
            fallback=move || view! {
                <div class="flex min-h-screen items-center justify-center bg-background text-sm text-foreground/70">
                    "Chargement du tableau de bord…"
                </div>
            }
        >
            <Show
                when=move || is_authenticated.get()
                fallback=move || view! {
                    <div class="flex min-h-screen items-center justify-center bg-background text-sm text-foreground/70">
                        "Redirection vers la page de connexion…"
                    </div>
                }
            >
                <div class="flex min-h-screen flex-col bg-background text-foreground">
                    <DashboardNavbar
                        on_open_settings=open_settings.clone()
                        on_open_workspace_modal=Callback::new({
                            let show = show_workspace_modal.clone();
                            move |_| show.set(true)
                        })
                    />
                    <main class="flex-1 bg-surface/40">
                        <div class="mx-auto w-full max-w-6xl px-4 py-8 sm:px-6 lg:px-12">
                            {move || children_store.with_value(|child| child())}
                        </div>
                    </main>
                    <DashboardFooter/>
                </div>
            </Show>
        </Show>
        <SettingsModal open=show_settings_modal/>
        <WorkspaceModal open=show_workspace_modal/>
    }
}
