use leptos::ev;
use leptos::prelude::*;

#[component]
pub fn FormControl(
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] hint: Option<String>,
    #[prop(optional, into)] error: Option<String>,
    #[prop(optional, into)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let classes = format!("flex flex-col gap-2 {}", class.unwrap_or_default())
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    view! {
        <div class=classes>
            {label.as_ref().map(|label_text| view! {
                <label class="text-sm font-medium text-foreground/80">{label_text.clone()}</label>
            })}
            <div class="flex flex-col gap-2">
                {children()}
            </div>
            {hint.as_ref().map(|hint_text| view! {
                <p class="text-xs text-foreground/60">{hint_text.clone()}</p>
            })}
            {error.as_ref().map(|error_text| view! {
                <p class="text-xs font-medium text-danger">{error_text.clone()}</p>
            })}
        </div>
    }
}

#[component]
pub fn InputField(
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional, into)] name: Option<String>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(default = "text")] input_type: &'static str,
    #[prop(optional, into)] value: Option<String>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] on_input: Option<Callback<ev::Event>>,
) -> impl IntoView {
    let classes = format!(
        "h-11 w-full rounded-xl border border-border/60 bg-surface px-4 text-sm text-foreground placeholder:text-foreground/40 shadow-inner shadow-black/10 transition focus:border-brand/60 focus:outline-none focus:ring-2 focus:ring-brand/40 disabled:opacity-60 disabled:pointer-events-none {}",
        class.unwrap_or_default()
    )
    .split_whitespace()
    .collect::<Vec<_>>()
    .join(" ");

    let handler = on_input.clone();

    view! {
        <input
            class=classes
            id=id.unwrap_or_default()
            name=name.unwrap_or_default()
            placeholder=placeholder.unwrap_or_default()
            r#type=input_type
            value=value.unwrap_or_default()
            disabled=disabled
            on:input=move |ev| {
                if let Some(cb) = handler.clone() {
                    cb.run(ev);
                }
            }
        />
    }
}
