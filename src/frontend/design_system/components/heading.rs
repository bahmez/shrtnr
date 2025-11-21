use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeadingLevel {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

impl HeadingLevel {
    fn classes(self) -> &'static str {
        match self {
            HeadingLevel::H1 => "text-3xl font-semibold tracking-tight text-foreground sm:text-4xl",
            HeadingLevel::H2 => {
                "text-2xl font-semibold tracking-tight text-foreground/95 sm:text-3xl"
            }
            HeadingLevel::H3 => "text-xl font-semibold tracking-tight text-foreground/90",
            HeadingLevel::H4 => "text-lg font-semibold tracking-tight text-foreground/85",
            HeadingLevel::H5 => {
                "text-base font-semibold uppercase tracking-wide text-foreground/70"
            }
            HeadingLevel::H6 => "text-sm font-semibold uppercase tracking-wide text-foreground/60",
        }
    }
}

#[component]
pub fn Heading(
    #[prop(default = HeadingLevel::H2)] level: HeadingLevel,
    #[prop(optional, into)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let level = level;
    let classes = format!("{} {}", level.classes(), class.unwrap_or_default())
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    let heading: AnyView = match level {
        HeadingLevel::H1 => view! { <h1 class=classes.clone()>{children()}</h1> }.into_any(),
        HeadingLevel::H2 => view! { <h2 class=classes.clone()>{children()}</h2> }.into_any(),
        HeadingLevel::H3 => view! { <h3 class=classes.clone()>{children()}</h3> }.into_any(),
        HeadingLevel::H4 => view! { <h4 class=classes.clone()>{children()}</h4> }.into_any(),
        HeadingLevel::H5 => view! { <h5 class=classes.clone()>{children()}</h5> }.into_any(),
        HeadingLevel::H6 => view! { <h6 class=classes.clone()>{children()}</h6> }.into_any(),
    };

    heading
}
