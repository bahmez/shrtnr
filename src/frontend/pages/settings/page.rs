use crate::frontend::{
    design_system::{
        Button, ButtonSize, ButtonVariant, Card, CardBody, CardHeader, FormControl, Heading,
        HeadingLevel, InputField, Text, TextTone,
    },
    layouts::DashboardLayout,
    state::{use_auth_store, AuthStore},
};
use leptos::ev;
use leptos::prelude::*;

#[cfg(feature = "hydrate")]
use leptos::task::spawn_local;
#[cfg(feature = "hydrate")]
use crate::frontend::pages::settings::update_profile;
#[cfg(feature = "hydrate")]
use crate::frontend::pages::auth::UpdateProfilePayload;

fn create_submit_handler(
    display_name: RwSignal<String>,
    email: RwSignal<String>,
    saving: RwSignal<bool>,
    error_message: RwSignal<Option<String>>,
    success_message: RwSignal<Option<String>>,
    auth_store: AuthStore,
) -> impl Fn(ev::SubmitEvent) + Clone {
    move |ev: ev::SubmitEvent| {
        ev.prevent_default();

        if saving.get_untracked() {
            return;
        }

        #[cfg(feature = "hydrate")]
        {
            let display_name_value = display_name.get_untracked();
            let email_value = email.get_untracked();

            error_message.set(None);
            success_message.set(None);

            saving.set(true);

            let payload = UpdateProfilePayload {
                name: (!display_name_value.trim().is_empty()).then(|| display_name_value.trim().to_string()),
                email: (!email_value.trim().is_empty()).then(|| email_value.trim().to_string()),
            };

            let saving = saving.clone();
            let error_message = error_message.clone();
            let success_message = success_message.clone();
            let auth_store = auth_store.clone();

            spawn_local(async move {
                let access_token = match auth_store.access_token() {
                    Some(token) => token,
                    None => {
                        error_message.set(Some("Session expirée, veuillez vous reconnecter.".into()));
                        saving.set(false);
                        return;
                    }
                };

                match update_profile(&access_token, &payload).await {
                    Ok(user) => {
                        auth_store.update_user(user);
                        success_message.set(Some("Profil mis à jour avec succès.".into()));
                    }
                    Err(err) => {
                        error_message.set(Some(err));
                    }
                }

                saving.set(false);
            });
        }
    }
}

#[component]
pub fn SettingsPage() -> impl IntoView {
    let auth_store = use_auth_store();
    let user_signal = auth_store.user();

    let display_name = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());

    let saving = RwSignal::new(false);
    let error_message = RwSignal::new(None::<String>);
    let success_message = RwSignal::new(None::<String>);

    Effect::new({
        let display_name = display_name.clone();
        let email = email.clone();
        move |_| {
            if let Some(user) = user_signal.get() {
                display_name.set(user.name.clone().unwrap_or_default());
                email.set(user.email.clone());
            }
        }
    });

    let handle_name_input = {
        let display_name = display_name.clone();
        Callback::new(move |ev: ev::Event| {
            display_name.set(event_target_value(&ev));
        })
    };

    let handle_email_input = {
        let email = email.clone();
        Callback::new(move |ev: ev::Event| {
            email.set(event_target_value(&ev));
        })
    };

    let handle_submit = create_submit_handler(
        display_name.clone(),
        email.clone(),
        saving.clone(),
        error_message.clone(),
        success_message.clone(),
        auth_store.clone(),
    );

    view! {
        <DashboardLayout>
            <section class="grid gap-6">
                <div>
                    <Heading level=HeadingLevel::H1>"Paramètres"</Heading>
                </div>

                <Card>
                    <CardHeader>
                        <Heading level=HeadingLevel::H2>"Profil"</Heading>
                    </CardHeader>
                    <CardBody>
                        <form
                            class="grid gap-5"
                            on:submit=handle_submit
                        >
                            <FormControl label="Nom complet">
                                <InputField
                                    id="display-name"
                                    name="display_name"
                                    placeholder="Votre nom"
                                    value=display_name
                                    on_input=handle_name_input
                                />
                            </FormControl>

                            <FormControl label="Adresse email" hint="Votre email servira aux notifications d'équipe.">
                                <InputField
                                    id="email"
                                    name="email"
                                    input_type="email"
                                    placeholder="you@company.com"
                                    value=email
                                    on_input=handle_email_input
                                />
                            </FormControl>

                            {move || error_message.get().map(|msg| view! {
                                <Text tone=TextTone::Danger class="rounded-xl border border-danger/40 bg-danger/10 px-4 py-3 text-sm">
                                    {msg}
                                </Text>
                            })}

                            {move || success_message.get().map(|msg| view! {
                                <Text tone=TextTone::Success class="rounded-xl border border-success/40 bg-success/10 px-4 py-3 text-sm">
                                    {msg}
                                </Text>
                            })}

                            <div class="flex items-center justify-end gap-3 pt-2">
                                <Button
                                    variant=ButtonVariant::Primary
                                    size=ButtonSize::Sm
                                    class="min-w-[120px]"
                                >
                                    {move || if saving.get() { "Enregistrement…" } else { "Enregistrer" }}
                                </Button>
                            </div>
                        </form>
                    </CardBody>
                </Card>
            </section>
        </DashboardLayout>
    }
}
