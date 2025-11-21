use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TextTone {
    #[default]
    Default,
    Muted,
    Subtle,
    Success,
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
