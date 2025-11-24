#[cfg(feature = "hydrate")]
use super::client::{
    add_workspace_member, list_workspace_members, remove_workspace_member, update_workspace,
};
#[cfg(feature = "hydrate")]
use super::client::WorkspaceMemberWithUserResponse;

use crate::frontend::design_system::{
    Button, ButtonSize, ButtonVariant, FormControl, Heading, HeadingLevel, InputField, Text,
    TextTone,
};
#[cfg(feature = "hydrate")]
use crate::frontend::design_system::{Badge, BadgeVariant, Card, CardBody};
use crate::frontend::layouts::DashboardLayout;

#[cfg(feature = "hydrate")]
use crate::frontend::state::{use_auth_store, use_workspace_store};
#[cfg(not(feature = "hydrate"))]
use crate::frontend::state::WorkspaceSummary;

use leptos::ev::SubmitEvent;
use leptos::prelude::*;
#[cfg(feature = "hydrate")]
use leptos::task::spawn_local;

#[cfg(feature = "hydrate")]
fn members_section(
    members: RwSignal<Vec<WorkspaceMemberWithUserResponse>>,
    members_loading: RwSignal<bool>,
    members_error: RwSignal<Option<String>>,
    remove_loading: RwSignal<Option<String>>,
    remove_error: RwSignal<Option<String>>,
    remove_success: RwSignal<Option<String>>,
    show_invite_modal: RwSignal<bool>,
) -> impl IntoView {
    let auth_store = use_auth_store();
    let workspace_store = use_workspace_store();

    view! {
        <div class="rounded-2xl border border-border/60 bg-surface/80 p-6 shadow-lg shadow-black/5 backdrop-blur">
            <div class="mb-6 flex items-center justify-between">
                <div class="flex flex-col gap-2">
                    <Heading level=HeadingLevel::H2 class="text-lg">
                        "Membres du workspace"
                    </Heading>
                    <Text tone=TextTone::Muted class="text-sm">
                        "Liste de tous les membres du workspace avec leurs rôles"
                    </Text>
                </div>
                <Button
                    variant=ButtonVariant::Primary
                    size=ButtonSize::Sm
                    on:click=move |_| show_invite_modal.set(true)
                >
                    "Inviter un utilisateur"
                </Button>
            </div>

            {move || remove_error.get().map(|err| view! {
                <div class="mb-4 rounded-lg border border-danger/20 bg-danger/10 p-3">
                    <Text tone=TextTone::Danger class="text-sm">{err}</Text>
                </div>
            })}

            {move || remove_success.get().map(|msg| view! {
                <div class="mb-4 rounded-lg border border-success/20 bg-success/10 p-3">
                    <Text class="text-sm text-success">{msg}</Text>
                </div>
            })}

            {move || {
                if members_loading.get() {
                    view! {
                        <div class="py-8 text-center">
                            <Text tone=TextTone::Muted>"Chargement des membres..."</Text>
                        </div>
                    }
                    .into_view()
                } else if let Some(err) = members_error.get() {
                    view! {
                        <div class="rounded-lg border border-danger/20 bg-danger/10 p-3">
                            <Text tone=TextTone::Danger class="text-sm">{err}</Text>
                        </div>
                    }
                    .into_view()
                } else if members.get().is_empty() {
                    view! {
                        <div class="py-8 text-center">
                            <Text tone=TextTone::Muted>"Aucun membre dans ce workspace"</Text>
                        </div>
                    }
                    .into_view()
                } else {
                    view! {
                        <div class="mt-6 grid gap-4">
                            <For
                                each=move || members.get()
                                key=|member| member.user_email.clone()
                                children=move |member| {
                                    let role = member.role.clone();
                                    let user_email = member.user_email.clone();
                                    let user_name = member.user_name.clone();
                                    let user_id = member.user_id.clone();

                                    let display_name = user_name.unwrap_or_else(|| user_email.clone());

                                    let role_display = match role.as_str() {
                                        "Owner" => "Propriétaire".to_string(),
                                        "Admin" => "Administrateur".to_string(),
                                        "Member" => "Membre".to_string(),
                                        _ => role.clone(),
                                    };
                                    let role_variant = match role.as_str() {
                                        "Owner" => BadgeVariant::Solid,
                                        "Admin" => BadgeVariant::Subtle,
                                        _ => BadgeVariant::Outline,
                                    };

                                    let is_owner = role == "Owner";
                                    let user_id_for_disabled = user_id.clone();
                                    let user_id_for_text = user_id.clone();
                                    let user_id_for_click = user_id.clone();

                                    view! {
                                        <Card>
                                            <CardBody class="pt-6">
                                                <div class="flex items-center justify-between">
                                                    <div class="flex flex-col gap-1">
                                                        <Text class="font-medium">
                                                            {display_name}
                                                        </Text>
                                                        <Text tone=TextTone::Muted class="text-sm">
                                                            {user_email}
                                                        </Text>
                                                    </div>
                                                    <div class="flex items-center gap-3">
                                                        <Badge variant=role_variant>
                                                            {role_display}
                                                        </Badge>
                                                        {if !is_owner {
                                                            Some(view! {
                                                                <Button
                                                                    variant=ButtonVariant::Outline
                                                                    size=ButtonSize::Sm
                                                                    class="border-danger/60 text-danger hover:bg-danger/10 focus-visible:ring-danger/40"
                                                                    prop:disabled=move || {
                                                                        remove_loading.get().as_ref().map(|id| id == &user_id_for_disabled).unwrap_or(false)
                                                                    }
                                                                    on:click=move |_| {
                                                                        let user_id_clone = user_id_for_click.clone();

                                                                        let Some(token) = auth_store.access_token() else {
                                                                            remove_error.set(Some("Connexion requise".to_string()));
                                                                            return;
                                                                        };

                                                                        let Some(workspace_id) = workspace_store.selected_id().get() else {
                                                                            remove_error.set(Some("Aucun workspace sélectionné".to_string()));
                                                                            return;
                                                                        };

                                                                        remove_loading.set(Some(user_id_clone.clone()));
                                                                        remove_error.set(None);
                                                                        remove_success.set(None);

                                                                        let token_clone = token.clone();
                                                                        let workspace_id_clone = workspace_id.clone();

                                                                        spawn_local(async move {
                                                                            match remove_workspace_member(
                                                                                &workspace_id_clone,
                                                                                &token_clone,
                                                                                &user_id_clone,
                                                                            )
                                                                            .await
                                                                            {
                                                                                Ok(_) => {
                                                                                    remove_success.set(Some(
                                                                                        "Membre supprimé avec succès".to_string(),
                                                                                    ));
                                                                                    remove_loading.set(None);

                                                                                    // Refresh the members list
                                                                                    members_loading.set(true);
                                                                                    match list_workspace_members(&workspace_id_clone, &token_clone).await {
                                                                                        Ok(members_list) => {
                                                                                            members.set(members_list);
                                                                                            members_loading.set(false);
                                                                                        }
                                                                                        Err(_) => {
                                                                                            members_loading.set(false);
                                                                                        }
                                                                                    }
                                                                                }
                                                                                Err(err) => {
                                                                                    remove_error.set(Some(err));
                                                                                    remove_loading.set(None);
                                                                                }
                                                                            }
                                                                        });
                                                                    }
                                                                >
                                                                    {move || {
                                                                        let is_removing = remove_loading.get().as_ref().map(|id| id == &user_id_for_text).unwrap_or(false);
                                                                        if is_removing { "Suppression..." } else { "Supprimer" }
                                                                    }}
                                                                </Button>
                                                            })
                                                        } else {
                                                            None
                                                        }}
                                                    </div>
                                                </div>
                                            </CardBody>
                                        </Card>
                                    }
                                }
                            />
                        </div>
                    }
                    .into_view()
                }
            }}
        </div>
    }
}

#[cfg(not(feature = "hydrate"))]
fn members_section(
    _members: RwSignal<Vec<()>>,
    _members_loading: RwSignal<bool>,
    _members_error: RwSignal<Option<String>>,
    _remove_loading: RwSignal<Option<String>>,
    _remove_error: RwSignal<Option<String>>,
    _remove_success: RwSignal<Option<String>>,
    _show_invite_modal: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <div class="rounded-2xl border border-border/60 bg-surface/80 p-6 shadow-lg shadow-black/5 backdrop-blur">
            <div class="mb-6 flex flex-col gap-2">
                <Heading level=HeadingLevel::H2 class="text-lg">
                    "Membres du workspace"
                </Heading>
                <Text tone=TextTone::Muted class="text-sm">
                    "Liste de tous les membres du workspace avec leurs rôles"
                </Text>
            </div>
            <div class="py-8 text-center">
                <Text tone=TextTone::Muted>"Fonctionnalité non disponible en mode SSR"</Text>
            </div>
        </div>
    }
}

#[cfg(feature = "hydrate")]
fn invite_modal(
    show_invite_modal: RwSignal<bool>,
    invite_email: RwSignal<String>,
    invite_role: RwSignal<String>,
    invite_loading: RwSignal<bool>,
    invite_error: RwSignal<Option<String>>,
    invite_success: RwSignal<Option<String>>,
    members: RwSignal<Vec<WorkspaceMemberWithUserResponse>>,
    members_loading: RwSignal<bool>,
) -> impl IntoView {
    let auth_store = use_auth_store();
    let workspace_store = use_workspace_store();

    view! {
        {move || {
            if show_invite_modal.get() {
                Some(view! {
                    <div
                        class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur"
                        on:click=move |_| {
                            show_invite_modal.set(false);
                            invite_error.set(None);
                            invite_success.set(None);
                        }
                    >
                        <div
                            class="mx-4 w-full max-w-md rounded-2xl border border-border/60 bg-surface p-6 shadow-xl"
                            on:click=move |e| {
                                e.stop_propagation();
                            }
                        >
                            <div class="mb-6 flex items-center justify-between">
                                <Heading level=HeadingLevel::H2 class="text-lg">
                                    "Inviter un utilisateur"
                                </Heading>
                                <button
                                    class="text-foreground/60 hover:text-foreground transition"
                                    on:click=move |_| {
                                        show_invite_modal.set(false);
                                        invite_error.set(None);
                                        invite_success.set(None);
                                    }
                                >
                                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                        <line x1="18" y1="6" x2="6" y2="18"></line>
                                        <line x1="6" y1="6" x2="18" y2="18"></line>
                                    </svg>
                                </button>
                            </div>

                            <Text tone=TextTone::Muted class="mb-6 text-sm">
                                "Ajoutez un membre à votre workspace en saisissant son email"
                            </Text>

                            <form on:submit=move |ev: SubmitEvent| {
                                ev.prevent_default();

                                let email_value = invite_email.get();
                                let role_value = invite_role.get();

                                if email_value.is_empty() {
                                    invite_error.set(Some("L'email est requis".to_string()));
                                    return;
                                }

                                // Validate that user is not inviting themselves
                                if let Some(current_user) = auth_store.user().get() {
                                    if email_value.to_lowercase() == current_user.email.to_lowercase() {
                                        invite_error.set(Some("Vous ne pouvez pas vous inviter vous-même".to_string()));
                                        return;
                                    }
                                }

                                let Some(token) = auth_store.access_token() else {
                                    invite_error.set(Some("Connexion requise".to_string()));
                                    return;
                                };

                                let Some(workspace_id) = workspace_store.selected_id().get() else {
                                    invite_error.set(Some("Aucun workspace sélectionné".to_string()));
                                    return;
                                };

                                invite_loading.set(true);
                                invite_error.set(None);
                                invite_success.set(None);

                                let email_clone = email_value.clone();
                                let role_clone = role_value.clone();
                                let token_clone = token.clone();
                                let workspace_id_clone = workspace_id.clone();

                                spawn_local(async move {
                                    match add_workspace_member(
                                        &workspace_id_clone,
                                        &token_clone,
                                        Some(&email_clone),
                                        None,
                                        &role_clone,
                                    )
                                    .await
                                    {
                                        Ok(_) => {
                                            invite_success.set(Some(
                                                "Utilisateur invité avec succès".to_string(),
                                            ));
                                            invite_email.set(String::new());
                                            invite_loading.set(false);

                                            // Refresh the members list
                                            members_loading.set(true);
                                            match list_workspace_members(&workspace_id_clone, &token_clone).await {
                                                Ok(members_list) => {
                                                    members.set(members_list);
                                                    members_loading.set(false);

                                                    // Close modal after successful invite and list refresh
                                                    show_invite_modal.set(false);
                                                    invite_success.set(None);
                                                }
                                                Err(_) => {
                                                    members_loading.set(false);
                                                    show_invite_modal.set(false);
                                                    invite_success.set(None);
                                                }
                                            }
                                        }
                                        Err(err) => {
                                            invite_error.set(Some(err));
                                            invite_loading.set(false);
                                        }
                                    }
                                });
                            } class="grid gap-6">
                                <FormControl
                                    label="Email"
                                    hint="Email de l'utilisateur à inviter"
                                >
                                    <InputField
                                        id="invite-email"
                                        name="email"
                                        input_type="email"
                                        placeholder="utilisateur@example.com"
                                        value=invite_email
                                        on_input=Callback::new(move |ev| invite_email.set(event_target_value(&ev)))
                                        prop:disabled=move || invite_loading.get()
                                    />
                                </FormControl>

                                <FormControl label="Rôle">
                                    <select
                                        class="h-11 w-full rounded-xl border border-border/60 bg-surface px-4 text-sm text-foreground shadow-inner shadow-black/10 transition focus:border-brand/60 focus:outline-none focus:ring-2 focus:ring-brand/40 disabled:opacity-60 disabled:pointer-events-none"
                                        prop:value=move || invite_role.get()
                                        on:change=move |ev| invite_role.set(event_target_value(&ev))
                                        prop:disabled=move || invite_loading.get()
                                    >
                                        <option value="Member">"Membre"</option>
                                        <option value="Admin">"Administrateur"</option>
                                    </select>
                                </FormControl>

                                {move || invite_error.get().map(|err| view! {
                                    <div class="rounded-lg border border-danger/20 bg-danger/10 p-3">
                                        <Text tone=TextTone::Danger class="text-sm">{err}</Text>
                                    </div>
                                })}

                                {move || invite_success.get().map(|msg| view! {
                                    <div class="rounded-lg border border-success/20 bg-success/10 p-3">
                                        <Text class="text-sm text-success">{msg}</Text>
                                    </div>
                                })}

                                <div class="flex gap-3">
                                    <Button
                                        variant=ButtonVariant::Secondary
                                        size=ButtonSize::Md
                                        full_width=true
                                        on:click=move |_| {
                                            show_invite_modal.set(false);
                                            invite_error.set(None);
                                            invite_success.set(None);
                                        }
                                    >
                                        "Annuler"
                                    </Button>
                                    <Button
                                        variant=ButtonVariant::Primary
                                        size=ButtonSize::Md
                                        prop:disabled=move || invite_loading.get()
                                        full_width=true
                                    >
                                        {move || if invite_loading.get() { "Invitation..." } else { "Inviter" }}
                                    </Button>
                                </div>
                            </form>
                        </div>
                    </div>
                })
            } else {
                None
            }
        }}
    }
}

#[cfg(not(feature = "hydrate"))]
fn invite_modal(
    _show_invite_modal: RwSignal<bool>,
    _invite_email: RwSignal<String>,
    _invite_role: RwSignal<String>,
    _invite_loading: RwSignal<bool>,
    _invite_error: RwSignal<Option<String>>,
    _invite_success: RwSignal<Option<String>>,
    _members: RwSignal<Vec<()>>,
    _members_loading: RwSignal<bool>,
) -> impl IntoView {
    view! {}
}

#[component]
pub fn SettingsPage() -> impl IntoView {
    #[cfg(feature = "hydrate")]
    let auth_store = use_auth_store();
    #[cfg(feature = "hydrate")]
    let workspace_store = use_workspace_store();

    let workspace_name = RwSignal::new(String::new());
    let loading = RwSignal::new(false);
    let error = RwSignal::new(Option::<String>::None);
    let success = RwSignal::new(Option::<String>::None);

    // Modal state
    let show_invite_modal = RwSignal::new(false);

    // Invitation d'utilisateur
    let invite_email = RwSignal::new(String::new());
    let invite_role = RwSignal::new("Member".to_string());
    let invite_loading = RwSignal::new(false);
    let invite_error = RwSignal::new(Option::<String>::None);
    let invite_success = RwSignal::new(Option::<String>::None);

    // Liste des membres
    #[cfg(feature = "hydrate")]
    let members: RwSignal<Vec<WorkspaceMemberWithUserResponse>> = RwSignal::new(Vec::new());
    #[cfg(not(feature = "hydrate"))]
    let members: RwSignal<Vec<()>> = RwSignal::new(Vec::new());

    #[cfg(feature = "hydrate")]
    let members_loading: RwSignal<bool> = RwSignal::new(false);
    #[cfg(not(feature = "hydrate"))]
    let members_loading = RwSignal::new(false);

    #[cfg(feature = "hydrate")]
    let members_error: RwSignal<Option<String>> = RwSignal::new(None);
    #[cfg(not(feature = "hydrate"))]
    let members_error = RwSignal::new(None);

    // Suppression de membre
    let remove_loading: RwSignal<Option<String>> = RwSignal::new(None);
    let remove_error: RwSignal<Option<String>> = RwSignal::new(None);
    let remove_success: RwSignal<Option<String>> = RwSignal::new(None);

    // Initialiser le nom du workspace depuis le store et charger les membres
    #[cfg(feature = "hydrate")]
    {
        let selected_workspace = workspace_store.selected_workspace();

        Effect::new(move |_| {
            if let Some(workspace) = selected_workspace.get() {
                workspace_name.set(workspace.name.clone());

                if let Some(token) = auth_store.access_token() {
                    members_loading.set(true);
                    members_error.set(None);

                    let workspace_id = workspace.id.clone();
                    let token_clone = token.clone();

                    spawn_local(async move {
                        match list_workspace_members(&workspace_id, &token_clone).await {
                            Ok(members_list) => {
                                members.set(members_list);
                                members_loading.set(false);
                            }
                            Err(err) => {
                                members_error.set(Some(err));
                                members_loading.set(false);
                            }
                        }
                    });
                }
            }
        });
    }

    #[cfg(feature = "hydrate")]
    let selected_workspace = workspace_store.selected_workspace();
    #[cfg(not(feature = "hydrate"))]
    let selected_workspace = Signal::derive(|| None::<WorkspaceSummary>);

    let has_workspace = Signal::derive(move || selected_workspace.get().is_some());
    let show_invite_modal_clone = show_invite_modal;

    view! {
        <DashboardLayout>
            <div class="mx-auto flex w-full max-w-4xl flex-col gap-8">
                <div class="flex flex-col gap-2">
                    <Heading level=HeadingLevel::H1>
                        "Paramètres du workspace"
                    </Heading>
                    <Text tone=TextTone::Muted>
                        "Gérez les paramètres et les membres de votre workspace"
                    </Text>
                </div>

                {move || {
                    if !has_workspace.get() {
                        view! {
                            <div class="rounded-2xl border border-border/60 bg-surface/80 p-12 shadow-lg shadow-black/5 backdrop-blur">
                                <div class="flex flex-col items-center justify-center gap-6 text-center">
                                    <div class="flex flex-col gap-2">
                                        <Heading level=HeadingLevel::H2 class="text-xl">
                                            "Aucun workspace sélectionné"
                                        </Heading>
                                        <Text tone=TextTone::Muted class="text-sm">
                                            "Sélectionnez un workspace existant ou créez-en un nouveau pour commencer"
                                        </Text>
                                    </div>
                                </div>
                            </div>
                        }
                        .into_view()
                    } else {
                        view! {
                            <div class="grid gap-8">
                                // Section modification du workspace
                                <div class="rounded-2xl border border-border/60 bg-surface/80 p-6 shadow-lg shadow-black/5 backdrop-blur">
                                    <div class="mb-6 flex flex-col gap-2">
                                        <Heading level=HeadingLevel::H2 class="text-lg">
                                            "Informations du workspace"
                                        </Heading>
                                        <Text tone=TextTone::Muted class="text-sm">
                                            "Modifiez le nom de votre workspace"
                                        </Text>
                                    </div>

                        <form on:submit=move |ev: SubmitEvent| {
                            ev.prevent_default();
                            #[cfg(feature = "hydrate")]
                            {
                                let workspace_name_value = workspace_name.get();
                                let Some(token) = auth_store.access_token() else {
                                    error.set(Some("Connexion requise".to_string()));
                                    return;
                                };

                                let Some(workspace_id) = workspace_store.selected_id().get() else {
                                    error.set(Some("Aucun workspace sélectionné".to_string()));
                                    return;
                                };

                                loading.set(true);
                                error.set(None);
                                success.set(None);

                                let workspace_name_clone = workspace_name_value.clone();
                                let token_clone = token.clone();
                                let workspace_id_clone = workspace_id.clone();

                                spawn_local(async move {
                                    let name = if workspace_name_clone.is_empty() {
                                        None
                                    } else {
                                        Some(workspace_name_clone)
                                    };

                                    match update_workspace(&workspace_id_clone, &token_clone, name).await {
                                        Ok(updated) => {
                                            workspace_store.update_workspace_in_store(updated);
                                            success.set(Some("Workspace mis à jour avec succès".to_string()));
                                            loading.set(false);
                                        }
                                        Err(err) => {
                                            error.set(Some(err));
                                            loading.set(false);
                                        }
                                    }
                                });
                            }
                        } class="grid gap-6">
                            <FormControl label="Nom du workspace">
                                <InputField
                                    id="workspace-name"
                                    name="name"
                                    placeholder="Mon workspace"
                                    value=workspace_name
                                    on_input=Callback::new(move |ev| workspace_name.set(event_target_value(&ev)))
                                    prop:disabled=move || loading.get()
                                />
                            </FormControl>

                            {move || error.get().map(|err| view! {
                                <div class="rounded-lg border border-danger/20 bg-danger/10 p-3">
                                    <Text tone=TextTone::Danger class="text-sm">{err}</Text>
                                </div>
                            })}

                            {move || success.get().map(|msg| view! {
                                <div class="rounded-lg border border-success/20 bg-success/10 p-3">
                                    <Text class="text-sm text-success">{msg}</Text>
                                </div>
                            })}

                            <Button
                                variant=ButtonVariant::Primary
                                size=ButtonSize::Md
                                prop:disabled=move || loading.get()
                                full_width=false
                            >
                                {move || if loading.get() { "Enregistrement..." } else { "Enregistrer" }}
                            </Button>
                        </form>
                    </div>

                                // Section liste des membres avec modal d'invitation
                                {members_section(members, members_loading, members_error, remove_loading, remove_error, remove_success, show_invite_modal_clone)}
                            </div>
                        }
                        .into_view()
                    }
                }}
            </div>

            // Modal d'invitation
            {invite_modal(show_invite_modal, invite_email, invite_role, invite_loading, invite_error, invite_success, members, members_loading)}
        </DashboardLayout>
    }
}
