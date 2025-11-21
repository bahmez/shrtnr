use crate::frontend::{
    design_system::{Badge, BadgeVariant, Button, ButtonSize, ButtonVariant, Text, TextTone},
    state::use_auth_store,
};
use leptos::ev;
use leptos::prelude::*;
#[cfg(feature = "hydrate")]
use leptos::task::spawn_local;
#[cfg(feature = "hydrate")]
use leptos_router::hooks::use_navigate;
#[cfg(feature = "hydrate")]
use leptos_router::NavigateOptions;

const NAV_LINKS: [(&str, &str); 3] = [("Links", "#"), ("Analytics", "#"), ("Settings", "#")];

#[component]
pub fn DashboardNavbar(on_open_settings: Callback<()>) -> impl IntoView {
    let projects = RwSignal::new(vec![
        "Campagne Paid Q3".to_string(),
        "Newsletter Automne".to_string(),
        "Partenaires Retail".to_string(),
    ]);
    let selected_project = RwSignal::new("Campagne Paid Q3".to_string());
    let project_menu_open = RwSignal::new(false);
    let profile_menu_open = RwSignal::new(false);
    let mobile_menu_open = RwSignal::new(false);

    let toggle_project_menu = {
        let project_menu_open = project_menu_open;
        let profile_menu_open = profile_menu_open;
        let mobile_menu_open = mobile_menu_open;
        Callback::new(move |_: ev::MouseEvent| {
            profile_menu_open.set(false);
            mobile_menu_open.set(false);
            project_menu_open.update(|open| *open = !*open);
        })
    };

    let toggle_profile_menu = {
        let profile_menu_open = profile_menu_open;
        let project_menu_open = project_menu_open;
        let mobile_menu_open = mobile_menu_open;
        Callback::new(move |_: ev::MouseEvent| {
            project_menu_open.set(false);
            mobile_menu_open.set(false);
            profile_menu_open.update(|open| *open = !*open);
        })
    };

    let toggle_mobile_menu = {
        let mobile_menu_open = mobile_menu_open;
        let profile_menu_open = profile_menu_open;
        let project_menu_open = project_menu_open;
        Callback::new(move |_: ev::MouseEvent| {
            project_menu_open.set(false);
            profile_menu_open.set(false);
            mobile_menu_open.update(|open| *open = !*open);
        })
    };

    let create_new_project = {
        let projects = projects;
        let selected_project = selected_project;
        let project_menu_open = project_menu_open;
        Callback::new(move |_: ev::MouseEvent| {
            projects.update(|list| {
                let new_project = format!("Nouveau projet {}", list.len() + 1);
                selected_project.set(new_project.clone());
                list.push(new_project);
            });
            project_menu_open.set(false);
        })
    };

    let on_open_settings_dropdown = on_open_settings.clone();
    let open_settings = {
        let profile_menu_open = profile_menu_open;
        Callback::new(move |_: ev::MouseEvent| {
            profile_menu_open.set(false);
            on_open_settings_dropdown.run(());
        })
    };

    let on_open_settings_mobile = on_open_settings.clone();

    let auth_store = use_auth_store();
    let user_signal = auth_store.user();

    #[cfg(feature = "hydrate")]
    let navigate = use_navigate();
    #[cfg(feature = "hydrate")]
    let logout_action = {
        let auth_store = auth_store.clone();
        let navigate = navigate.clone();
        let profile_menu_open = profile_menu_open.clone();
        Callback::new(move |_| {
            profile_menu_open.set(false);
            let auth_store = auth_store.clone();
            let navigate = navigate.clone();
            spawn_local(async move {
                let _ = auth_store.logout().await;
                let _ = navigate(
                    "/",
                    NavigateOptions {
                        replace: true,
                        ..Default::default()
                    },
                );
            });
        })
    };
    #[cfg(not(feature = "hydrate"))]
    let logout_action = Callback::new(|_| {});

    let user_initials = Signal::derive(move || {
        user_signal.get().map(|user| {
            user.name
                .clone()
                .unwrap_or_else(|| user.email.clone())
                .split_whitespace()
                .filter_map(|part| part.chars().next())
                .take(2)
                .collect::<String>()
                .to_uppercase()
        })
    });

    view! {
        <header class="sticky top-0 z-40 border-b border-border/60 bg-background/80 backdrop-blur">
            <div class="mx-auto flex w-full max-w-6xl flex-col px-4 py-4 sm:px-6 lg:px-12">
                <div class="flex items-center justify-between gap-4">
                    <div class="flex items-center gap-4">
                        <div class="relative">
                            <Button
                                variant=ButtonVariant::Outline
                                size=ButtonSize::Sm
                                class="min-w-[190px] justify-between text-sm"
                                on_click=toggle_project_menu
                            >
                                <span class="flex items-center gap-2">
                                    <Badge variant=BadgeVariant::Subtle class="bg-brand/15 text-brand">
                                        "Projet"
                                    </Badge>
                                    <span class="font-medium text-foreground/90">
                                        {move || selected_project.get()}
                                    </span>
                                </span>
                                <span aria-hidden="true" class="text-xs text-foreground/60">"▾"</span>
                            </Button>

                            {move || project_menu_open.get().then(|| view! {
                                <div class="absolute left-0 z-30 mt-2 w-64 overflow-hidden rounded-xl border border-border/60 bg-surface shadow-lg shadow-black/10">
                                    <div class="flex flex-col divide-y divide-border/40">
                                        <div class="px-4 py-3">
                                            <Text tone=TextTone::Subtle class="text-xs uppercase tracking-wide">
                                                "Vos projets"
                                            </Text>
                                        </div>
                                        <div class="max-h-60 overflow-y-auto">
                                            {move || {
                                                projects
                                                    .get()
                                                    .into_iter()
                                                    .map(|project| {
                                                        let selected_project = selected_project;
                                                        let project_menu_open = project_menu_open;
                                                    let display_label = project.clone();
                                                    let select_value = project.clone();
                                                    let indicator_value = project;
                                                        view! {
                                                            <button
                                                                class="flex w-full items-center justify-between px-4 py-3 text-left text-sm text-foreground/90 transition hover:bg-surface-strong/70"
                                                                on:click=move |_| {
                                                                selected_project.set(select_value.clone());
                                                                    project_menu_open.set(false);
                                                                }
                                                            >
                                                            <span>{display_label.clone()}</span>
                                                                {move || {
                                                                let current = indicator_value.clone();
                                                                (selected_project.get() == current)
                                                                        .then(|| view! { <span class="text-xs text-brand">"●"</span> })
                                                                }}
                                                            </button>
                                                        }
                                                    })
                                                    .collect_view()
                                            }}
                                        </div>
                                        <div class="px-4 py-3">
                                            <Button
                                                variant=ButtonVariant::Ghost
                                                size=ButtonSize::Sm
                                                class="w-full justify-center"
                                                on_click=create_new_project
                                            >
                                                "+ Créer un projet"
                                            </Button>
                                        </div>
                                    </div>
                                </div>
                            })}
                        </div>

                        <nav class="hidden items-center gap-6 text-sm md:flex">
                            {NAV_LINKS
                                .iter()
                                .map(|(label, href)| {
                                    let label_text = *label;
                                    let href_value = *href;
                                    view! {
                                        <a
                                            href=href_value
                                            class="text-foreground/70 transition hover:text-foreground"
                                        >
                                            {label_text}
                                        </a>
                                    }
                                })
                                .collect_view()}
                        </nav>
                    </div>

                    <div class="flex items-center gap-2">
                        <Button
                            variant=ButtonVariant::Ghost
                            size=ButtonSize::Sm
                            class="md:hidden"
                            on_click=toggle_mobile_menu
                        >
                            <span class="sr-only">"Ouvrir le menu"</span>
                            <span aria-hidden="true" class="text-base">"☰"</span>
                        </Button>

                        <div class="relative">
                            <button
                                class="flex h-10 w-10 items-center justify-center rounded-full bg-brand/15 text-sm font-semibold text-brand transition hover:bg-brand/20 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-brand/40"
                                on:click=move |ev: ev::MouseEvent| {
                                    ev.stop_propagation();
                                    toggle_profile_menu.run(ev);
                                }
                            >
                                {move || user_initials.get().unwrap_or_else(|| "CM".into())}
                            </button>

                            {move || profile_menu_open.get().then(|| view! {
                                <div class="absolute right-0 z-30 mt-3 w-64 rounded-xl border border-border/60 bg-surface shadow-xl shadow-black/10">
                                    <div class="flex flex-col gap-1 p-4">
                                        {move || {
                                            user_signal
                                                .get()
                                                .map(|user| {
                                                    let display_name = user
                                                        .name
                                                        .clone()
                                                        .unwrap_or_else(|| user.email.clone());
                                                    let email_value = user.email.clone();
                                                    view! {
                                                        <div class="pb-3 flex flex-col gap-0.5">
                                                            <Text class="text-sm font-semibold text-foreground">
                                                                {display_name}
                                                            </Text>
                                                            <Text tone=TextTone::Subtle class="text-xs">
                                                                {email_value}
                                                            </Text>
                                                        </div>
                                                    }
                                                    .into_any()
                                                })
                                                .unwrap_or_else(|| {
                                                    view! {
                                                        <div class="pb-3">
                                                            <Text tone=TextTone::Muted class="text-xs uppercase tracking-wide">
                                                                "Mon compte"
                                                            </Text>
                                                        </div>
                                                    }
                                                    .into_any()
                                                })
                                        }}
                                        <Button
                                            variant=ButtonVariant::Ghost
                                            size=ButtonSize::Sm
                                            class="justify-start"
                                            on_click=open_settings.clone()
                                        >
                                            "Settings"
                                        </Button>
                                        <Button
                                            variant=ButtonVariant::Ghost
                                            size=ButtonSize::Sm
                                            class="justify-start text-danger hover:text-danger"
                                            on_click=logout_action.clone()
                                        >
                                            "Logout"
                                        </Button>
                                    </div>
                                </div>
                            })}
                        </div>
                    </div>
                </div>

                {move || mobile_menu_open.get().then(|| view! {
                    <div class="mt-4 grid gap-3 rounded-2xl border border-border/60 bg-surface p-4 md:hidden">
                        <div class="flex flex-col gap-2">
                            <Text tone=TextTone::Subtle class="text-xs uppercase tracking-wide">
                                "Navigation"
                            </Text>
                            {NAV_LINKS
                                .iter()
                                .map(|(label, href)| {
                                    let mobile_menu_open = mobile_menu_open;
                                    let href_value = *href;
                                    let label_value = *label;
                                    view! {
                                        <a
                                            href=href_value
                                            class="rounded-xl px-3 py-2 text-sm text-foreground/80 transition hover:bg-surface-strong"
                                            on:click=move |_| mobile_menu_open.set(false)
                                        >
                                            {label_value}
                                        </a>
                                    }
                                })
                                .collect_view()}
                        </div>

                        <div class="flex flex-col gap-2">
                            <Text tone=TextTone::Subtle class="text-xs uppercase tracking-wide">
                                "Compte"
                            </Text>
                            {move || {
                                user_signal.get().map(|user| {
                                    let display_name = user
                                        .name
                                        .clone()
                                        .unwrap_or_else(|| user.email.clone());
                                    let email_value = user.email.clone();
                                    view! {
                                        <div class="rounded-xl border border-border/60 bg-surface px-3 py-2 text-left">
                                            <Text class="text-sm font-medium text-foreground">
                                                {display_name}
                                            </Text>
                                            <Text tone=TextTone::Subtle class="text-xs">
                                                {email_value}
                                            </Text>
                                        </div>
                                    }
                                    .into_any()
                                })
                            }}
                            <Button
                                variant=ButtonVariant::Outline
                                size=ButtonSize::Sm
                                class="justify-center"
                                on_click=Callback::new(move |_: ev::MouseEvent| {
                                    mobile_menu_open.set(false);
                                    on_open_settings_mobile.run(());
                                })
                            >
                                "Ouvrir les settings"
                            </Button>
                            <Button
                                variant=ButtonVariant::Ghost
                                size=ButtonSize::Sm
                                class="justify-center text-danger hover:text-danger"
                                on_click=Callback::new(move |event: ev::MouseEvent| {
                                    mobile_menu_open.set(false);
                                    logout_action.run(event);
                                })
                            >
                                "Logout"
                            </Button>
                        </div>
                    </div>
                })}
            </div>
        </header>
    }
}
