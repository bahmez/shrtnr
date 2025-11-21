use crate::frontend::{
    design_system::{Button, ButtonSize, ButtonVariant, FormControl, InputField, Text, TextTone},
    state::{use_auth_store, use_workspace_store},
};
use leptos::ev;
use leptos::prelude::*;
#[cfg(feature = "hydrate")]
use leptos::task::spawn_local;

#[component]
pub fn WorkspaceModal(open: RwSignal<bool>) -> impl IntoView {
    #[allow(unused_variables)]
    let auth_store = use_auth_store();
    #[allow(unused_variables)]
    let workspace_store = use_workspace_store();

    let name = RwSignal::new(String::new());
    let saving = RwSignal::new(false);
    let error_message = RwSignal::new(Option::<String>::None);
    let success_message = RwSignal::new(Option::<String>::None);

    let close_modal = {
        let open = open;
        Callback::new(move |_| {
            open.set(false);
        })
    };

    {
        let open = open.clone();
        let name = name.clone();
        let error_message = error_message.clone();
        let success_message = success_message.clone();
        Effect::new(move |_| {
            if open.get() {
                name.set(String::new());
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
                let name_handle = name.clone();
                let saving_handle = saving.clone();
                let error_handle = error_message.clone();
                let success_handle = success_message.clone();
                #[cfg(feature = "hydrate")]
                let auth_store_handle = auth_store.clone();
                #[cfg(feature = "hydrate")]
                let workspace_store_handle = workspace_store.clone();

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
                                        "Nouveau workspace"
                                    </Text>
                                    <Text tone=TextTone::Subtle>
                                        "Créez un espace de travail pour organiser vos campagnes."
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

                                    let trimmed_name =
                                        name_handle.get_untracked().trim().to_string();
                                    if trimmed_name.len() < 3 {
                                        error_handle.set(Some(
                                            "Le nom doit contenir au moins 3 caractères.".into(),
                                        ));
                                        return;
                                    }

                                    #[cfg(feature = "hydrate")]
                                    {
                                        if let Some(user) = auth_store_handle.user().get_untracked() {
                                            saving_handle.set(true);
                                            error_handle.set(None);
                                            success_handle.set(None);
                                            let open = open_handle.clone();
                                            let saving = saving_handle.clone();
                                            let success = success_handle.clone();
                                            let error = error_handle.clone();
                                            let workspace_store = workspace_store_handle.clone();
                                            let name_value = trimmed_name.clone();
                                            spawn_local(async move {
                                                match workspace_store
                                                    .create_workspace(
                                                        user.id.clone(),
                                                        name_value.clone(),
                                                    )
                                                {
                                                    Some(create_future) => {
                                                        match create_future.await {
                                                            Ok(_) => {
                                                                success.set(Some(
                                                                    "Workspace créé avec succès."
                                                                        .into(),
                                                                ));
                                                                open.set(false);
                                                            }
                                                            Err(err) => {
                                                                error.set(Some(err));
                                                            }
                                                        }
                                                    }
                                                    None => {
                                                        error.set(Some(
                                                            "Session expirée, veuillez vous reconnecter."
                                                                .into(),
                                                        ));
                                                    }
                                                }
                                                saving.set(false);
                                            });
                                        } else {
                                            error_handle.set(Some(
                                                "Impossible de créer un workspace sans utilisateur."
                                                    .into(),
                                            ));
                                        }
                                    }

                                    #[cfg(not(feature = "hydrate"))]
                                    {
                                        open_handle.set(false);
                                    }
                                }
                            >
                                <FormControl label="Nom du workspace">
                                    <InputField
                                        id="workspace-name-input"
                                        name="workspace_name"
                                        placeholder="Campagne Q3 Retail"
                                        value=name_handle
                                        on_input=Callback::new({
                                            let name = name_handle.clone();
                                            move |ev: ev::Event| name.set(event_target_value(&ev))
                                        })
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
                                        "Fermer"
                                    </Button>
                                    <Button
                                        variant=ButtonVariant::Primary
                                        size=ButtonSize::Sm
                                        class="min-w-[140px]"
                                    >
                                        {move || if saving_handle.get() {
                                            "Création…"
                                        } else {
                                            "Créer"
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
