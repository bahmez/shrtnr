//! Composant Text du design system.
//!
//! Fournit un composant de texte avec différents tons pour la hiérarchie visuelle.

use leptos::prelude::*;

/// Tons de texte disponibles pour la hiérarchie visuelle.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TextTone {
    /// Texte par défaut (opacité 90%)
    #[default]
    Default,
    /// Texte atténué (opacité 70%)
    Muted,
    /// Texte subtil (opacité 60%)
    Subtle,
    /// Texte de succès (couleur success)
    Success,
    /// Texte d'erreur (couleur danger)
    Danger,
}

impl TextTone {
    fn classes(self) -> &'static str {
        match self {
            TextTone::Default => "text-foreground/90",
            TextTone::Muted => "text-foreground/70",
            TextTone::Subtle => "text-foreground/60",
            TextTone::Success => "text-success",
            TextTone::Danger => "text-danger",
        }
    }
}

/// Composant Text pour afficher du texte avec différents tons.
///
/// Utilisé pour créer une hiérarchie visuelle dans le texte.
///
/// # Props
///
/// * `tone` - Ton du texte (Default, Muted, Subtle, Success, Danger)
/// * `class` - Classes CSS supplémentaires (optionnel)
/// * `children` - Contenu textuel
#[component]
pub fn Text(
    #[prop(optional)] tone: TextTone,
    #[prop(optional, into)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let classes = format!(
        "leading-relaxed {} {}",
        tone.classes(),
        class.unwrap_or_default()
    )
    .split_whitespace()
    .collect::<Vec<_>>()
    .join(" ");

    view! { <p class=classes>{children()}</p> }
}
