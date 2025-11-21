use leptos::ev;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Outline,
    Ghost,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ButtonSize {
    #[default]
    Md,
    Sm,
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
