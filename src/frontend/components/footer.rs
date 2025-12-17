//! Composant Footer pour la page d'accueil.
//!
//! Affiche le pied de page avec les liens de navigation, informations légales,
//! et liens vers les différentes sections du site.

use crate::frontend::design_system::{Heading, HeadingLevel, Text, TextTone};
use leptos::prelude::*;

/// Composant Footer de l'application.
///
/// Affiche :
/// - Description de l'application
/// - Liens de navigation (Produit, Ressources, Entreprise)
/// - Informations légales (copyright, conditions, confidentialité)
///
/// Utilisé principalement sur la landing page.
#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="border-t border-border/60 bg-background">
            <div class="mx-auto flex w-full max-w-6xl flex-col gap-10 px-6 py-12 lg:flex-row lg:items-start lg:justify-between lg:px-12">
                <div class="flex max-w-md flex-col gap-3">
                    <Heading level=HeadingLevel::H4 class="text-lg font-semibold tracking-tight">
                        "shrtnr"
                    </Heading>
                    <Text tone=TextTone::Subtle>
                        "La plateforme qui vous aide à transformer des URLs longues en expériences courtes et mesurables pour vos équipes marketing."
                    </Text>
                </div>
                <div class="grid gap-8 text-sm text-foreground/80 sm:grid-cols-3">
                    <div class="flex flex-col gap-2">
                        <span class="font-medium text-foreground">"Produit"</span>
                        <a href="#features" class="transition hover:text-foreground">"Fonctionnalités"</a>
                        <a href="#workflow" class="transition hover:text-foreground">"Automatisations"</a>
                        <a href="#pricing" class="transition hover:text-foreground">"Tarifs"</a>
                    </div>
                    <div class="flex flex-col gap-2">
                        <span class="font-medium text-foreground">"Ressources"</span>
                        <a class="transition hover:text-foreground" href="#">"Guide d'onboarding"</a>
                        <a class="transition hover:text-foreground" href="#">"API docs"</a>
                        <a class="transition hover:text-foreground" href="#">"Statut"</a>
                    </div>
                    <div class="flex flex-col gap-2">
                        <span class="font-medium text-foreground">"Entreprise"</span>
                        <a class="transition hover:text-foreground" href="#">"À propos"</a>
                        <a class="transition hover:text-foreground" href="#">"Presse"</a>
                        <a class="transition hover:text-foreground" href="#">"Contact"</a>
                    </div>
                </div>
            </div>
            <div class="border-t border-border/60 bg-background/60">
                <div class="mx-auto flex w-full max-w-6xl flex-col gap-4 px-6 py-6 text-sm text-foreground/60 lg:flex-row lg:items-center lg:justify-between lg:px-12">
                    <span>"© 2025 shrtnr. Tous droits réservés."</span>
                    <div class="flex flex-wrap gap-4">
                        <a class="transition hover:text-foreground" href="#">"Conditions"</a>
                        <a class="transition hover:text-foreground" href="#">"Confidentialité"</a>
                        <a class="transition hover:text-foreground" href="#">"Sécurité"</a>
                    </div>
                </div>
            </div>
        </footer>
    }
}
