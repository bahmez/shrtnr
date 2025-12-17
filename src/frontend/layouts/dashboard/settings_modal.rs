//! Modal de paramètres du profil utilisateur.
//!
//! Permet aux utilisateurs de modifier leur nom et email.

use crate::frontend::{
    design_system::{Button, ButtonSize, ButtonVariant, FormControl, InputField, Text, TextTone},
    state::use_auth_store,
};
use leptos::prelude::Effect;
#[cfg(feature = "hydrate")]
use leptos::task::spawn_local;
use leptos::{ev, prelude::*};

/// Modal pour modifier les paramètres du profil utilisateur.
///
/// Affiche un formulaire permettant de :
/// - Modifier le nom d'affichage
/// - Modifier l'adresse email
/// - Sauvegarder les modifications via l'API
/// - Afficher les erreurs ou messages de succès
///
/// # Arguments
///
/// * `open` - Signal contrôlant l'ouverture/fermeture du modal
#[component]
pub fn SettingsModal(open: RwSignal<bool>) -> impl IntoView {
    let auth_store = use_auth_store();
    let user_signal = auth_store.user();

    let display_name = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let saving = RwSignal::new(false);
    let error_message = RwSignal::new(Option::<String>::None);
    let success_message = RwSignal::new(Option::<String>::None);

    let close_modal = {
        let open = open;
        Callback::new(move |_| {
            open.set(false);
        })
    };

    let handle_name_input = {
        let display_name = display_name;
        Callback::new(move |ev: ev::Event| {
            display_name.set(event_target_value(&ev));
        })
    };

    let handle_email_input = {
        let email = email;
        Callback::new(move |ev: ev::Event| {
            email.set(event_target_value(&ev));
        })
    };

    {
        let open = open.clone();
        let display_name = display_name.clone();
        let email = email.clone();
        let error_message = error_message.clone();
        let success_message = success_message.clone();
        Effect::new(move |_| {
            if open.get() {
                if let Some(user) = user_signal.get() {
                    display_name.set(user.name.clone().unwrap_or_default());
                    email.set(user.email.clone());
                }
                error_message.set(None);
                success_message.set(None);
            }
        });
    }

    view! {
        {move || {
            if !open.get() {
                None
            } else {
                let open_handle = open.clone();
                let display_handle = display_name.clone();
                let email_handle = email.clone();
                let saving_handle = saving.clone();
                let error_handle = error_message.clone();
                let success_handle = success_message.clone();
                #[cfg(feature = "hydrate")]
                let auth_store_handle = auth_store.clone();

                Some(view! {
                <div
                    class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-6 backdrop-blur"
                    on:click=move |_| open_handle.set(false)
                >
                    <div
                        class="relative w-full max-w-lg rounded-2xl border border-border/60 bg-background p-6 shadow-2xl shadow-black/20"
                        on:click=move |ev: ev::MouseEvent| {
                            ev.stop_propagation();
                        }
                    >
                        <div class="flex items-start justify-between gap-4">
                            <div class="flex flex-col gap-1">
                                <Text class="text-lg font-semibold text-foreground">
                                    "Paramètres du profil"
                                </Text>
                                <Text tone=TextTone::Subtle>
                                    "Mettez à jour votre nom ou votre adresse email pour synchroniser vos espaces de travail."
                                </Text>
                            </div>
                            <Button
                                variant=ButtonVariant::Ghost
                                size=ButtonSize::Sm
                                class="h-9 w-9 items-center justify-center rounded-full"
                                on_click=close_modal.clone()
                            >
                                <span aria-hidden="true" class="text-lg">"×"</span>
                            </Button>
                        </div>

                        <form
                            class="mt-6 grid gap-5"
                            on:submit=move |ev: ev::SubmitEvent| {
                                ev.prevent_default();

                                if saving_handle.get_untracked() {
                                    return;
                                }

                                #[cfg(feature = "hydrate")]
                                {
                                    error_handle.set(None);
                                    success_handle.set(None);

                                    let email_value = email_handle.get_untracked();
                                    if email_value.trim().is_empty() {
                                        error_handle
                                            .set(Some("Veuillez renseigner une adresse email.".into()));
                                        return;
                                    }

                                    saving_handle.set(true);
                                    let name_value = display_handle.get_untracked();
                                    let open = open_handle.clone();
                                    let saving = saving_handle.clone();
                                    let error_message = error_handle.clone();
                                    let success_message = success_handle.clone();
                                    let auth_store = auth_store_handle.clone();

                                    spawn_local(async move {
                                        let trimmed_name = name_value.trim().to_string();
                                        let name_payload =
                                            (!trimmed_name.is_empty()).then_some(trimmed_name);
                                        let email_payload = Some(email_value.trim().to_string());

                                        match auth_store
                                            .submit_profile(name_payload, email_payload)
                                            .await
                                        {
                                            Ok(_) => {
                                                success_message.set(Some("Profil mis à jour.".into()));
                                                open.set(false);
                                            }
                                            Err(err) => {
                                                error_message.set(Some(err));
                                            }
                                        }

                                        saving.set(false);
                                    });
                                }

                                #[cfg(not(feature = "hydrate"))]
                                {
                                    open_handle.set(false);
                                }
                            }
                        >
                            <FormControl label="Nom complet">
                                <InputField
                                    id="display-name"
                                    name="display_name"
                                    placeholder="Votre nom"
                                    value=display_handle
                                    on_input=handle_name_input.clone()
                                />
                            </FormControl>

                            <FormControl label="Adresse email" hint="Votre email servira aux notifications d'équipe.">
                                <InputField
                                    id="email"
                                    name="email"
                                    input_type="email"
                                    placeholder="you@company.com"
                                    value=email_handle
                                    on_input=handle_email_input.clone()
                                />
                            </FormControl>

                            {move || error_handle.get().map(|message| view! {
                                <Text tone=TextTone::Danger class="rounded-xl border border-danger/40 bg-danger/10 px-4 py-3 text-sm">
                                    {message}
                                </Text>
                            })}

                            {move || success_handle.get().map(|message| view! {
                                <Text tone=TextTone::Success class="rounded-xl border border-success/40 bg-success/10 px-4 py-3 text-sm">
                                    {message}
                                </Text>
                            })}

                            <div class="flex items-center justify-end gap-3 pt-2">
                                <Button
                                    variant=ButtonVariant::Ghost
                                    size=ButtonSize::Sm
                                    on_click=close_modal.clone()
                                >
                                    "Annuler"
                                </Button>
                                <Button
                                    variant=ButtonVariant::Primary
                                    size=ButtonSize::Sm
                                    class="min-w-[120px]"
                                >
                                    {move || if saving_handle.get() {
                                        "Enregistrement…"
                                    } else {
                                        "Enregistrer"
                                    }}
                                </Button>
                            </div>
                        </form>
                    </div>
                </div>
            })
            }
        }}
    }
}
