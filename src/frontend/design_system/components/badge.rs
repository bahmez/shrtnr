use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum BadgeVariant {
    #[default]
    Solid,
    Subtle,
    Outline,
}

#[component]
pub fn Badge(
    #[prop(default = BadgeVariant::Solid)] variant: BadgeVariant,
    #[prop(optional, into)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let base = "inline-flex items-center rounded-full px-3 py-1 text-xs font-medium uppercase tracking-wide";
    let variant_classes = match variant {
        BadgeVariant::Solid => "bg-brand/90 text-brand-foreground",
        BadgeVariant::Subtle => "bg-brand/10 text-brand",
        BadgeVariant::Outline => "border border-brand/50 text-brand",
    };
    let classes = format!("{} {} {}", base, variant_classes, class.unwrap_or_default())
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    view! { <span class=classes>{children()}</span> }
}
