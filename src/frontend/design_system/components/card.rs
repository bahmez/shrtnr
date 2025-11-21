use leptos::prelude::*;

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
