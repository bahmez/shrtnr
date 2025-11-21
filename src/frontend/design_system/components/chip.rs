use leptos::ev;
use leptos::prelude::*;

#[component]
pub fn Chip(
    #[prop(optional)] leading_icon: Option<AnyView>,
    #[prop(optional)] removable: bool,
    #[prop(optional, into)] on_remove: Option<Callback<ev::MouseEvent>>,
    #[prop(optional, into)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let classes = format!(
        "inline-flex items-center gap-2 rounded-full border border-border/60 bg-surface px-3 py-1 text-xs text-foreground/80 shadow-inner shadow-black/10 {}",
        class.unwrap_or_default()
    )
    .split_whitespace()
    .collect::<Vec<_>>()
    .join(" ");

    let on_remove_cb = on_remove.clone();

    let leading_icon_view = leading_icon.map(|icon| icon.into_view());

    view! {
        <span class=classes>
            {leading_icon_view}
            <span>{children()}</span>
            {removable.then(|| view! {
                <button
                    type="button"
                    class="ml-1 inline-flex h-4 w-4 items-center justify-center rounded-full bg-transparent text-foreground/50 hover:text-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-brand/60"
                    on:click=move |ev| {
                        if let Some(cb) = on_remove_cb.clone() {
                            cb.run(ev);
                        }
                    }
                >
                    <span aria-hidden="true">"×"</span>
                </button>
            })}
        </span>
    }
}
