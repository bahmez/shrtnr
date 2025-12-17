//! Composant Button du design system.
//!
//! Fournit un composant bouton réutilisable avec différentes variantes et tailles.

use leptos::ev;
use leptos::prelude::*;

/// Variantes de style pour le composant Button.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ButtonVariant {
    /// Style principal (brand color)
    #[default]
    Primary,
    /// Style secondaire (surface forte)
    Secondary,
    /// Style avec bordure (outline)
    Outline,
    /// Style minimal (ghost, transparent)
    Ghost,
}

/// Tailles disponibles pour le composant Button.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ButtonSize {
    /// Taille moyenne (par défaut)
    #[default]
    Md,
    /// Taille petite
    Sm,
    /// Taille grande
    Lg,
}

impl ButtonVariant {
    fn to_classes(self) -> &'static str {
        match self {
            ButtonVariant::Primary => "bg-brand text-brand-foreground shadow-sm hover:bg-brand-muted focus-visible:ring-brand",
            ButtonVariant::Secondary => "bg-surface-strong text-foreground shadow-sm hover:bg-surface focus-visible:ring-brand",
            ButtonVariant::Outline => "border border-border bg-transparent text-foreground hover:bg-surface focus-visible:ring-brand/60",
            ButtonVariant::Ghost => "bg-transparent text-foreground/80 hover:bg-surface focus-visible:ring-brand/40",
        }
    }
}

impl ButtonSize {
    fn to_classes(self) -> &'static str {
        match self {
            ButtonSize::Sm => "h-8 px-3 text-xs",
            ButtonSize::Md => "h-10 px-4 text-sm",
            ButtonSize::Lg => "h-12 px-6 text-base",
        }
    }
}

/// Composant Button réutilisable du design system.
///
/// Bouton stylisé avec support de différentes variantes, tailles, et états.
///
/// # Props
///
/// * `variant` - Style du bouton (Primary, Secondary, Outline, Ghost)
/// * `size` - Taille du bouton (Sm, Md, Lg)
/// * `class` - Classes CSS supplémentaires (optionnel)
/// * `disabled` - Désactive le bouton
/// * `full_width` - Le bouton prend toute la largeur disponible
/// * `on_click` - Callback appelé lors du clic
/// * `children` - Contenu du bouton (texte, icônes, etc.)
///
/// # Exemple
///
/// ```rust,no_run
/// use shrtnr::frontend::design_system::{Button, ButtonVariant, ButtonSize};
///
/// view! {
///     <Button variant=ButtonVariant::Primary size=ButtonSize::Md on_click=move |_| {
///         println!("Clicked!");
///     }>
///         "Cliquez-moi"
///     </Button>
/// }
/// ```
#[component]
pub fn Button(
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional)] size: ButtonSize,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] full_width: bool,
    #[prop(optional, into)] on_click: Option<Callback<ev::MouseEvent>>,
    children: Children,
) -> impl IntoView {
    let base_classes = "inline-flex items-center gap-2 font-medium rounded-lg transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-offset-background disabled:opacity-60 disabled:pointer-events-none";
    let width_classes = if full_width {
        "w-full justify-center"
    } else {
        ""
    };
    let user_classes = class.unwrap_or_default();
    let final_classes = format!(
        "{} {} {} {} {}",
        base_classes,
        variant.to_classes(),
        size.to_classes(),
        width_classes,
        user_classes
    )
    .split_whitespace()
    .collect::<Vec<_>>()
    .join(" ");

    let handler = on_click.clone();

    view! {
        <button
            class=final_classes
            disabled=disabled
            on:click=move |ev| {
                if let Some(cb) = handler.clone() {
                    cb.run(ev);
                }
            }
        >
            {children()}
        </button>
    }
}
