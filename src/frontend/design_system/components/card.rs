//! Composants Card du design system.
//!
//! Fournit des composants de carte réutilisables avec header, body, et footer.

use leptos::prelude::*;

/// Composant Card pour afficher du contenu dans une carte stylisée.
///
/// # Props
///
/// * `class` - Classes CSS supplémentaires (optionnel)
/// * `children` - Contenu de la carte
#[component]
pub fn Card(#[prop(optional, into)] class: Option<String>, children: Children) -> impl IntoView {
    let user_classes = class.unwrap_or_default();
    let classes = format!(
        "rounded-2xl border border-border/60 bg-surface/60 backdrop-blur-md shadow-lg shadow-black/5 transition-shadow hover:shadow-xl hover:shadow-black/10 {}",
        user_classes
    )
    .split_whitespace()
    .collect::<Vec<_>>()
    .join(" ");

    view! {
        <section class=classes>
            {children()}
        </section>
    }
}

/// Composant CardHeader pour l'en-tête d'une carte.
///
/// # Props
///
/// * `class` - Classes CSS supplémentaires (optionnel)
/// * `children` - Contenu de l'en-tête
#[component]
pub fn CardHeader(
    #[prop(optional, into)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let classes = format!(
        "flex flex-col gap-2 px-6 pt-6 {}",
        class.unwrap_or_default()
    )
    .split_whitespace()
    .collect::<Vec<_>>()
    .join(" ");

    view! { <header class=classes>{children()}</header> }
}

/// Composant CardBody pour le corps d'une carte.
///
/// # Props
///
/// * `class` - Classes CSS supplémentaires (optionnel)
/// * `children` - Contenu du corps
#[component]
pub fn CardBody(
    #[prop(optional, into)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let classes = format!(
        "px-6 pb-6 text-sm text-foreground/90 {}",
        class.unwrap_or_default()
    )
    .split_whitespace()
    .collect::<Vec<_>>()
    .join(" ");

    view! { <div class=classes>{children()}</div> }
}

/// Composant CardFooter pour le pied d'une carte.
///
/// # Props
///
/// * `class` - Classes CSS supplémentaires (optionnel)
/// * `children` - Contenu du pied
#[component]
pub fn CardFooter(
    #[prop(optional, into)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let classes = format!(
        "px-6 pb-6 pt-4 border-t border-border/50 flex items-center gap-4 {}",
        class.unwrap_or_default()
    )
    .split_whitespace()
    .collect::<Vec<_>>()
    .join(" ");

    view! { <footer class=classes>{children()}</footer> }
}
