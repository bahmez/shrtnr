//! Composant Navigation pour la page d'accueil.
//!
//! Affiche la barre de navigation principale avec le logo, les liens de navigation
//! et les boutons d'action (connexion, inscription).

use crate::frontend::design_system::{
    Button, ButtonSize, ButtonVariant, Heading, HeadingLevel, Text, TextTone,
};
use leptos::{ev, prelude::*};
#[cfg(feature = "hydrate")]
use leptos_router::{hooks::use_navigate, NavigateOptions};

/// Composant Navigation de l'application.
///
/// Affiche :
/// - Logo et nom de l'application
/// - Liens de navigation (Fonctionnalités, Flux, Témoignages, Tarifs)
/// - Boutons d'action (Se connecter, Commencer)
///
/// La navigation est sticky (reste visible lors du scroll) et utilise
/// un backdrop blur pour un effet moderne.
#[component]
pub fn Navigation() -> impl IntoView {
    #[cfg(feature = "hydrate")]
    let navigate = use_navigate();

    #[cfg(feature = "hydrate")]
    let login_on_click = {
        let navigate = navigate.clone();
        Callback::new(move |_event: ev::MouseEvent| {
            let _ = navigate("/login", NavigateOptions::default());
        })
    };
    #[cfg(not(feature = "hydrate"))]
    let login_on_click = Callback::new(|_: ev::MouseEvent| {});

    #[cfg(feature = "hydrate")]
    let register_on_click = {
        let navigate = navigate.clone();
        Callback::new(move |_event: ev::MouseEvent| {
            let _ = navigate("/register", NavigateOptions::default());
        })
    };
    #[cfg(not(feature = "hydrate"))]
    let register_on_click = Callback::new(|_: ev::MouseEvent| {});

    view! {
        <header class="sticky top-0 z-30 border-b border-border/60 bg-background/80 backdrop-blur">
            <nav class="mx-auto flex w-full max-w-6xl items-center justify-between px-6 py-5 lg:px-12">
                <a href="#hero" class="flex items-center gap-3 transition hover:opacity-90">
                    <span class="flex h-10 w-10 items-center justify-center rounded-lg bg-brand/15 font-semibold text-brand">
                        "sh"
                    </span>
                    <span class="flex flex-col leading-tight">
                        <Heading level=HeadingLevel::H4 class="text-lg font-semibold tracking-tight">
                            "shrtnr"
                        </Heading>
                        <Text tone=TextTone::Subtle class="text-xs uppercase tracking-widest">
                            "URL shortener"
                        </Text>
                    </span>
                </a>
                <div class="hidden items-center gap-6 text-sm text-foreground/80 md:flex">
                    <a href="#features" class="transition hover:text-foreground">"Fonctionnalités"</a>
                    <a href="#workflow" class="transition hover:text-foreground">"Flux"</a>
                    <a href="#testimonials" class="transition hover:text-foreground">"Témoignages"</a>
                    <a href="#pricing" class="transition hover:text-foreground">"Tarifs"</a>
                </div>
                <div class="flex items-center gap-3">
                    <Button
                        variant=ButtonVariant::Ghost
                        size=ButtonSize::Sm
                        class="hidden md:inline-flex"
                        on_click=login_on_click
                    >
                        "Se connecter"
                    </Button>
                    <Button
                        variant=ButtonVariant::Primary
                        size=ButtonSize::Sm
                        on_click=register_on_click
                    >
                        "Commencer"
                    </Button>
                </div>
            </nav>
        </header>
    }
}
