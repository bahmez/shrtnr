#[cfg(feature = "hydrate")]
use super::{post_login, LoginPayload};
use crate::frontend::design_system::{
    Badge, BadgeVariant, Button, ButtonSize, ButtonVariant, FormControl, Heading, HeadingLevel,
    InputField, Text, TextTone,
};
#[cfg(feature = "hydrate")]
use crate::frontend::state::use_auth_store;
use leptos::ev::{self, SubmitEvent};
use leptos::prelude::*;
#[cfg(feature = "hydrate")]
use leptos::task::spawn_local;
use leptos_router::components::A;
#[cfg(feature = "hydrate")]
use leptos_router::hooks::{use_navigate, use_query_map};
#[cfg(feature = "hydrate")]
use leptos_router::NavigateOptions;

#[component]
pub fn LoginPage() -> impl IntoView {
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let loading = RwSignal::new(false);
    let error = RwSignal::new(Option::<String>::None);
    let success = RwSignal::new(Option::<String>::None);

    #[cfg(feature = "hydrate")]
    let nav = use_navigate();
    #[cfg(feature = "hydrate")]
    let query = use_query_map();
    #[cfg(feature = "hydrate")]
    let auth_store = use_auth_store();

    let on_email_input = {
        let email = email.clone();
        Callback::new(move |ev: ev::Event| email.set(event_target_value(&ev)))
    };

    let on_password_input = {
        let password = password.clone();
        Callback::new(move |ev: ev::Event| password.set(event_target_value(&ev)))
    };

    let submit = {
        let email = email.clone();
        let password = password.clone();
        let loading = loading.clone();
        let error = error.clone();
        let success = success.clone();
        #[cfg(feature = "hydrate")]
        let nav = nav.clone();
        #[cfg(feature = "hydrate")]
        let query = query.clone();
        #[cfg(feature = "hydrate")]
        let auth_store = auth_store.clone();

        move |ev: SubmitEvent| {
            ev.prevent_default();
            if loading.get_untracked() {
                return;
            }

            let email_value = email.get_untracked();
            let password_value = password.get_untracked();

            if email_value.is_empty() || password_value.is_empty() {
                error.set(Some(
                    "Veuillez renseigner votre email et votre mot de passe.".into(),
                ));
                return;
            }

            error.set(None);
            success.set(None);

            #[cfg(feature = "hydrate")]
            {
                loading.set(true);
                let payload = LoginPayload {
                    email: email_value,
                    password: password_value,
                };
                let loading = loading.clone();
                let error = error.clone();
                let success = success.clone();
                let nav = nav.clone();
                let query = query.clone();
                let auth_store = auth_store.clone();

                spawn_local(async move {
                    match post_login(&payload).await {
                        Ok(response) => {
                            let redirect_target = query
                                .with(|params| {
                                    params.get("redirect").and_then(|value| {
                                        value.starts_with('/').then(|| value.clone())
                                    })
                                })
                                .unwrap_or_else(|| "/workspace".to_string());

                            if let Err(store_err) = auth_store.apply_auth_response(&response) {
                                error.set(Some(store_err));
                            } else {
                                success.set(Some("Connexion réussie. Redirection…".into()));
                                let _ = nav(
                                    &redirect_target,
                                    NavigateOptions {
                                        replace: true,
                                        ..Default::default()
                                    },
                                );
                            }
                        }
                        Err(message) => {
                            error.set(Some(message));
                        }
                    }
                    loading.set(false);
                });
            }
        }
    };

    view! {
        <div class="mx-auto flex min-h-screen w-full max-w-md flex-col justify-center gap-8 px-6 py-12 lg:px-8">
            <div class="flex flex-col gap-3 text-center">
                <Badge variant=BadgeVariant::Subtle class="mx-auto w-fit">
                    "Espace sécurisé"
                </Badge>
                <Heading level=HeadingLevel::H2 class="text-balance">
                    "Connexion à shrtnr"
                </Heading>
                <Text tone=TextTone::Muted>
                    "Accédez à vos projets et gérez vos liens courts en toute sécurité."
                </Text>
            </div>

            <form class="grid gap-6 rounded-2xl border border-border/60 bg-surface/80 p-6 shadow-lg shadow-black/5 backdrop-blur" on:submit=submit>
                <FormControl label="Email">
                    <InputField
                        id="login-email"
                        input_type="email"
                        name="email"
                        placeholder="vous@entreprise.com"
                        value=email
                        on_input=on_email_input
                    />
                </FormControl>

                <FormControl label="Mot de passe">
                    <InputField
                        id="login-password"
                        input_type="password"
                        name="password"
                        placeholder="••••••••"
                        value=password
                        on_input=on_password_input
                    />
                </FormControl>

                {move || error.get().map(|message| view! {
                    <Text tone=TextTone::Danger class="rounded-xl border border-danger/40 bg-danger/10 px-4 py-3 text-sm">
                        {message}
                    </Text>
                })}

                {move || success.get().map(|message| view! {
                    <Text tone=TextTone::Success class="rounded-xl border border-success/40 bg-success/10 px-4 py-3 text-sm">
                        {message}
                    </Text>
                })}

                <Button
                    variant=ButtonVariant::Primary
                    size=ButtonSize::Lg
                    full_width=true
                    class="font-semibold"
                >
                    {move || if loading.get() {
                        "Connexion en cours…"
                    } else {
                        "Se connecter"
                    }}
                </Button>

                <Text tone=TextTone::Subtle class="text-center text-sm">
                    "Pas encore de compte ? "
                    <A href="/register">
                        <span class="font-semibold text-brand transition hover:text-brand-muted">
                            "Créer un compte"
                        </span>
                    </A>
                </Text>
            </form>
        </div>
    }
}
