//! Page d'inscription.
//!
//! Permet aux nouveaux utilisateurs de créer un compte avec email, mot de passe et nom.

#[cfg(feature = "hydrate")]
use super::{post_register, RegisterPayload};
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

/// Page d'inscription de l'application.
///
/// Affiche un formulaire d'inscription avec email, mot de passe et nom (optionnel).
/// Après une inscription réussie, redirige vers le dashboard.
#[component]
pub fn RegisterPage() -> impl IntoView {
    let name = RwSignal::new(String::new());
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

    let on_name_input = {
        #[cfg(feature = "hydrate")]
        let name = name.clone();
        Callback::new(move |ev: ev::Event| name.set(event_target_value(&ev)))
    };

    let on_email_input = {
        let email = email.clone();
        Callback::new(move |ev: ev::Event| email.set(event_target_value(&ev)))
    };

    let on_password_input = {
        let password = password.clone();
        Callback::new(move |ev: ev::Event| password.set(event_target_value(&ev)))
    };

    let submit = {
        #[cfg(feature = "hydrate")]
        let name = name.clone();
        let email = email.clone();
        let password = password.clone();
        let loading = loading.clone();
        let error = error.clone();
        let success = success.clone();
        #[cfg(feature = "hydrate")]
        let nav = nav.clone();

        move |ev: SubmitEvent| {
            ev.prevent_default();
            if loading.get_untracked() {
                return;
            }

            let email_value = email.get_untracked();
            let password_value = password.get_untracked();
            if email_value.is_empty() || password_value.is_empty() {
                error.set(Some(
                    "Veuillez fournir un email valide et un mot de passe sécurisé.".into(),
                ));
                return;
            }

            if password_value.len() < 8 {
                error.set(Some(
                    "Votre mot de passe doit contenir au moins 8 caractères.".into(),
                ));
                return;
            }

            error.set(None);
            success.set(None);

            #[cfg(feature = "hydrate")]
            {
                let name_value = name.get_untracked();
                loading.set(true);
                let payload = RegisterPayload {
                    email: email_value,
                    password: password_value,
                    name: (!name_value.trim().is_empty()).then_some(name_value),
                };

                let loading = loading.clone();
                let error = error.clone();
                let success = success.clone();
                let nav = nav.clone();
                let query = query.clone();
                let auth_store = auth_store.clone();

                spawn_local(async move {
                    match post_register(&payload).await {
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
                                success.set(Some("Compte créé avec succès. Redirection…".into()));
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
                <Badge variant=BadgeVariant::Solid class="mx-auto w-fit">
                    "Accès beta"
                </Badge>
                <Heading level=HeadingLevel::H2 class="text-balance">
                    "Créer un compte shrtnr"
                </Heading>
                <Text tone=TextTone::Muted>
                    "Centralisez vos liens courts, pilotez vos campagnes et partagez des insights actionnables avec vos équipes."
                </Text>
            </div>

            <form class="grid gap-6 rounded-2xl border border-border/60 bg-surface/80 p-6 shadow-lg shadow-black/5 backdrop-blur" on:submit=submit>
                <FormControl label="Nom (optionnel)">
                    <InputField
                        id="register-name"
                        name="name"
                        placeholder="Camille Martin"
                        value=name
                        on_input=on_name_input
                    />
                </FormControl>

                <FormControl label="Email professionnel">
                    <InputField
                        id="register-email"
                        input_type="email"
                        name="email"
                        placeholder="vous@entreprise.com"
                        value=email
                        on_input=on_email_input
                    />
                </FormControl>

                <FormControl
                    label="Mot de passe"
                    hint="Au moins 8 caractères, idéalement avec chiffres et symboles."
                >
                    <InputField
                        id="register-password"
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
                        "Création en cours…"
                    } else {
                        "Créer mon compte"
                    }}
                </Button>

                <Text tone=TextTone::Subtle class="text-center text-sm">
                    "Vous avez déjà un compte ? "
                    <A href="/login">
                        <span class="font-semibold text-brand transition hover:text-brand-muted">
                            "Se connecter"
                        </span>
                    </A>
                </Text>
            </form>
        </div>
    }
}
