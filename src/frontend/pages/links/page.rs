#[cfg(feature = "hydrate")]
use super::client::{create_link, delete_link, fetch_links, update_link};
use super::LinkItem;

#[cfg(feature = "hydrate")]
use crate::frontend::design_system::{
    Badge, BadgeVariant, Button, ButtonSize, ButtonVariant, Card, CardBody, FormControl, Heading,
    HeadingLevel, InputField, Text, TextTone,
};
#[cfg(not(feature = "hydrate"))]
use crate::frontend::design_system::{
    Button, ButtonVariant, Card, CardBody, Heading, HeadingLevel, Text, TextTone,
};
use crate::frontend::layouts::DashboardLayout;

#[cfg(feature = "hydrate")]
use crate::frontend::state::{use_auth_store, use_workspace_store};
#[cfg(not(feature = "hydrate"))]
use crate::frontend::state::WorkspaceSummary;

#[cfg(feature = "hydrate")]
use leptos::ev::SubmitEvent;
use leptos::prelude::*;
#[cfg(feature = "hydrate")]
use leptos::task::spawn_local;

// Create Link Modal
#[cfg(feature = "hydrate")]
fn create_link_modal(
    show_modal: RwSignal<bool>,
    form_url: RwSignal<String>,
    form_title: RwSignal<String>,
    form_custom_code: RwSignal<String>,
    form_loading: RwSignal<bool>,
    form_error: RwSignal<Option<String>>,
    links: RwSignal<Vec<LinkItem>>,
    links_loading: RwSignal<bool>,
    current_page: RwSignal<u64>,
) -> impl IntoView {
    let auth_store = use_auth_store();
    let workspace_store = use_workspace_store();

    view! {
        {move || {
            if show_modal.get() {
                Some(view! {
                    <div
                        class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur"
                        on:click=move |_| {
                            show_modal.set(false);
                            form_error.set(None);
                        }
                    >
                        <div
                            class="mx-4 w-full max-w-lg rounded-2xl border border-border/60 bg-surface p-6 shadow-xl"
                            on:click=move |e| {
                                e.stop_propagation();
                            }
                        >
                            <div class="mb-6 flex items-center justify-between">
                                <Heading level=HeadingLevel::H2 class="text-lg">
                                    "Creer un nouveau lien"
                                </Heading>
                                <button
                                    class="text-foreground/60 hover:text-foreground transition"
                                    on:click=move |_| {
                                        show_modal.set(false);
                                        form_error.set(None);
                                    }
                                >
                                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                        <line x1="18" y1="6" x2="6" y2="18"></line>
                                        <line x1="6" y1="6" x2="18" y2="18"></line>
                                    </svg>
                                </button>
                            </div>

                            <Text tone=TextTone::Muted class="mb-6 text-sm">
                                "Raccourcissez une URL longue en quelques secondes"
                            </Text>

                            <form on:submit=move |ev: SubmitEvent| {
                                ev.prevent_default();

                                let url_value = form_url.get();
                                let title_value = form_title.get();
                                let custom_code_value = form_custom_code.get();

                                if url_value.is_empty() {
                                    form_error.set(Some("L'URL est requise".to_string()));
                                    return;
                                }

                                if !url_value.starts_with("http://") && !url_value.starts_with("https://") {
                                    form_error.set(Some("L'URL doit commencer par http:// ou https://".to_string()));
                                    return;
                                }

                                let Some(token) = auth_store.access_token() else {
                                    form_error.set(Some("Connexion requise".to_string()));
                                    return;
                                };

                                let Some(workspace_id) = workspace_store.selected_id().get() else {
                                    form_error.set(Some("Aucun workspace selectionne".to_string()));
                                    return;
                                };

                                form_loading.set(true);
                                form_error.set(None);

                                let title = if title_value.is_empty() { None } else { Some(title_value) };
                                let custom_code = if custom_code_value.is_empty() { None } else { Some(custom_code_value) };

                                spawn_local(async move {
                                    match create_link(
                                        &token,
                                        url_value,
                                        workspace_id.clone(),
                                        title,
                                        custom_code,
                                        None,
                                    )
                                    .await
                                    {
                                        Ok(_) => {
                                            form_loading.set(false);
                                            form_url.set(String::new());
                                            form_title.set(String::new());
                                            form_custom_code.set(String::new());
                                            show_modal.set(false);

                                            // Refresh the links list
                                            links_loading.set(true);
                                            if let Ok(data) = fetch_links(&workspace_id, &token, current_page.get(), 10).await {
                                                links.set(data.links);
                                            }
                                            links_loading.set(false);
                                        }
                                        Err(err) => {
                                            form_error.set(Some(err));
                                            form_loading.set(false);
                                        }
                                    }
                                });
                            } class="grid gap-6">
                                <FormControl
                                    label="URL de destination"
                                    hint="L'URL longue que vous souhaitez raccourcir"
                                >
                                    <InputField
                                        id="create-url"
                                        name="url"
                                        input_type="url"
                                        placeholder="https://example.com/very/long/url"
                                        value=form_url
                                        on_input=Callback::new(move |ev| form_url.set(event_target_value(&ev)))
                                        prop:disabled=move || form_loading.get()
                                    />
                                </FormControl>

                                <FormControl
                                    label="Titre (optionnel)"
                                    hint="Un titre pour identifier facilement ce lien"
                                >
                                    <InputField
                                        id="create-title"
                                        name="title"
                                        placeholder="Mon lien important"
                                        value=form_title
                                        on_input=Callback::new(move |ev| form_title.set(event_target_value(&ev)))
                                        prop:disabled=move || form_loading.get()
                                    />
                                </FormControl>

                                <FormControl
                                    label="Code personnalise (optionnel)"
                                    hint="3-20 caracteres alphanumeriques, tirets ou underscores"
                                >
                                    <InputField
                                        id="create-custom-code"
                                        name="custom_code"
                                        placeholder="mon-lien"
                                        value=form_custom_code
                                        on_input=Callback::new(move |ev| form_custom_code.set(event_target_value(&ev)))
                                        prop:disabled=move || form_loading.get()
                                    />
                                </FormControl>

                                {move || form_error.get().map(|err| view! {
                                    <div class="rounded-lg border border-danger/20 bg-danger/10 p-3">
                                        <Text tone=TextTone::Danger class="text-sm">{err}</Text>
                                    </div>
                                })}

                                <div class="flex gap-3">
                                    <Button
                                        variant=ButtonVariant::Secondary
                                        size=ButtonSize::Md
                                        full_width=true
                                        on:click=move |_| {
                                            show_modal.set(false);
                                            form_error.set(None);
                                        }
                                    >
                                        "Annuler"
                                    </Button>
                                    <Button
                                        variant=ButtonVariant::Primary
                                        size=ButtonSize::Md
                                        prop:disabled=move || form_loading.get()
                                        full_width=true
                                    >
                                        {move || if form_loading.get() { "Creation..." } else { "Creer le lien" }}
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
fn create_link_modal(
    _show_modal: RwSignal<bool>,
    _form_url: RwSignal<String>,
    _form_title: RwSignal<String>,
    _form_custom_code: RwSignal<String>,
    _form_loading: RwSignal<bool>,
    _form_error: RwSignal<Option<String>>,
    _links: RwSignal<Vec<LinkItem>>,
    _links_loading: RwSignal<bool>,
    _current_page: RwSignal<u64>,
) -> impl IntoView {
    view! {}
}

// Edit Link Modal
#[cfg(feature = "hydrate")]
fn edit_link_modal(
    show_modal: RwSignal<bool>,
    editing_link: RwSignal<Option<LinkItem>>,
    edit_url: RwSignal<String>,
    edit_title: RwSignal<String>,
    edit_is_active: RwSignal<bool>,
    edit_loading: RwSignal<bool>,
    edit_error: RwSignal<Option<String>>,
    links: RwSignal<Vec<LinkItem>>,
    links_loading: RwSignal<bool>,
    current_page: RwSignal<u64>,
) -> impl IntoView {
    let auth_store = use_auth_store();
    let workspace_store = use_workspace_store();

    view! {
        {move || {
            if show_modal.get() && editing_link.get().is_some() {
                let link = editing_link.get().unwrap();
                Some(view! {
                    <div
                        class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur"
                        on:click=move |_| {
                            show_modal.set(false);
                            edit_error.set(None);
                            editing_link.set(None);
                        }
                    >
                        <div
                            class="mx-4 w-full max-w-lg rounded-2xl border border-border/60 bg-surface p-6 shadow-xl"
                            on:click=move |e| {
                                e.stop_propagation();
                            }
                        >
                            <div class="mb-6 flex items-center justify-between">
                                <Heading level=HeadingLevel::H2 class="text-lg">
                                    "Modifier le lien"
                                </Heading>
                                <button
                                    class="text-foreground/60 hover:text-foreground transition"
                                    on:click=move |_| {
                                        show_modal.set(false);
                                        edit_error.set(None);
                                        editing_link.set(None);
                                    }
                                >
                                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                        <line x1="18" y1="6" x2="6" y2="18"></line>
                                        <line x1="6" y1="6" x2="18" y2="18"></line>
                                    </svg>
                                </button>
                            </div>

                            <Text tone=TextTone::Muted class="mb-6 text-sm">
                                {format!("Code court: {}", link.short_code)}
                            </Text>

                            <form on:submit=move |ev: SubmitEvent| {
                                ev.prevent_default();

                                let Some(link) = editing_link.get() else {
                                    return;
                                };

                                let url_value = edit_url.get();
                                let title_value = edit_title.get();
                                let is_active_value = edit_is_active.get();

                                if url_value.is_empty() {
                                    edit_error.set(Some("L'URL est requise".to_string()));
                                    return;
                                }

                                if !url_value.starts_with("http://") && !url_value.starts_with("https://") {
                                    edit_error.set(Some("L'URL doit commencer par http:// ou https://".to_string()));
                                    return;
                                }

                                let Some(token) = auth_store.access_token() else {
                                    edit_error.set(Some("Connexion requise".to_string()));
                                    return;
                                };

                                let Some(workspace_id) = workspace_store.selected_id().get() else {
                                    edit_error.set(Some("Aucun workspace selectionne".to_string()));
                                    return;
                                };

                                edit_loading.set(true);
                                edit_error.set(None);

                                let link_id = link.id.clone();
                                let title = if title_value.is_empty() { None } else { Some(title_value) };

                                spawn_local(async move {
                                    match update_link(
                                        &token,
                                        &link_id,
                                        title,
                                        Some(url_value),
                                        Some(is_active_value),
                                    )
                                    .await
                                    {
                                        Ok(_) => {
                                            edit_loading.set(false);
                                            show_modal.set(false);
                                            editing_link.set(None);

                                            // Refresh the links list
                                            links_loading.set(true);
                                            if let Ok(data) = fetch_links(&workspace_id, &token, current_page.get(), 10).await {
                                                links.set(data.links);
                                            }
                                            links_loading.set(false);
                                        }
                                        Err(err) => {
                                            edit_error.set(Some(err));
                                            edit_loading.set(false);
                                        }
                                    }
                                });
                            } class="grid gap-6">
                                <FormControl
                                    label="URL de destination"
                                >
                                    <InputField
                                        id="edit-url"
                                        name="url"
                                        input_type="url"
                                        placeholder="https://example.com/very/long/url"
                                        value=edit_url
                                        on_input=Callback::new(move |ev| edit_url.set(event_target_value(&ev)))
                                        prop:disabled=move || edit_loading.get()
                                    />
                                </FormControl>

                                <FormControl
                                    label="Titre"
                                >
                                    <InputField
                                        id="edit-title"
                                        name="title"
                                        placeholder="Mon lien important"
                                        value=edit_title
                                        on_input=Callback::new(move |ev| edit_title.set(event_target_value(&ev)))
                                        prop:disabled=move || edit_loading.get()
                                    />
                                </FormControl>

                                <FormControl label="Statut">
                                    <div class="flex items-center gap-3">
                                        <button
                                            type="button"
                                            class=move || {
                                                let base = "relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-brand focus:ring-offset-2 focus:ring-offset-background";
                                                if edit_is_active.get() {
                                                    format!("{} bg-brand", base)
                                                } else {
                                                    format!("{} bg-foreground/20", base)
                                                }
                                            }
                                            on:click=move |_| edit_is_active.set(!edit_is_active.get())
                                            prop:disabled=move || edit_loading.get()
                                        >
                                            <span
                                                class=move || {
                                                    let base = "pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out";
                                                    if edit_is_active.get() {
                                                        format!("{} translate-x-5", base)
                                                    } else {
                                                        format!("{} translate-x-0", base)
                                                    }
                                                }
                                            ></span>
                                        </button>
                                        <Text tone=TextTone::Muted class="text-sm">
                                            {move || if edit_is_active.get() { "Actif" } else { "Inactif" }}
                                        </Text>
                                    </div>
                                </FormControl>

                                {move || edit_error.get().map(|err| view! {
                                    <div class="rounded-lg border border-danger/20 bg-danger/10 p-3">
                                        <Text tone=TextTone::Danger class="text-sm">{err}</Text>
                                    </div>
                                })}

                                <div class="flex gap-3">
                                    <Button
                                        variant=ButtonVariant::Secondary
                                        size=ButtonSize::Md
                                        full_width=true
                                        on:click=move |_| {
                                            show_modal.set(false);
                                            edit_error.set(None);
                                            editing_link.set(None);
                                        }
                                    >
                                        "Annuler"
                                    </Button>
                                    <Button
                                        variant=ButtonVariant::Primary
                                        size=ButtonSize::Md
                                        prop:disabled=move || edit_loading.get()
                                        full_width=true
                                    >
                                        {move || if edit_loading.get() { "Enregistrement..." } else { "Enregistrer" }}
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
fn edit_link_modal(
    _show_modal: RwSignal<bool>,
    _editing_link: RwSignal<Option<LinkItem>>,
    _edit_url: RwSignal<String>,
    _edit_title: RwSignal<String>,
    _edit_is_active: RwSignal<bool>,
    _edit_loading: RwSignal<bool>,
    _edit_error: RwSignal<Option<String>>,
    _links: RwSignal<Vec<LinkItem>>,
    _links_loading: RwSignal<bool>,
    _current_page: RwSignal<u64>,
) -> impl IntoView {
    view! {}
}

// Delete Confirmation Modal
#[cfg(feature = "hydrate")]
fn delete_confirm_modal(
    show_modal: RwSignal<bool>,
    deleting_link: RwSignal<Option<LinkItem>>,
    delete_loading: RwSignal<bool>,
    delete_error: RwSignal<Option<String>>,
    links: RwSignal<Vec<LinkItem>>,
    links_loading: RwSignal<bool>,
    current_page: RwSignal<u64>,
) -> impl IntoView {
    let auth_store = use_auth_store();
    let workspace_store = use_workspace_store();

    view! {
        {move || {
            if show_modal.get() && deleting_link.get().is_some() {
                let link = deleting_link.get().unwrap();
                let link_title = link.title.clone().unwrap_or_else(|| link.short_code.clone());
                Some(view! {
                    <div
                        class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur"
                        on:click=move |_| {
                            show_modal.set(false);
                            delete_error.set(None);
                            deleting_link.set(None);
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
                                    "Supprimer le lien"
                                </Heading>
                                <button
                                    class="text-foreground/60 hover:text-foreground transition"
                                    on:click=move |_| {
                                        show_modal.set(false);
                                        delete_error.set(None);
                                        deleting_link.set(None);
                                    }
                                >
                                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                        <line x1="18" y1="6" x2="6" y2="18"></line>
                                        <line x1="6" y1="6" x2="18" y2="18"></line>
                                    </svg>
                                </button>
                            </div>

                            <Text tone=TextTone::Muted class="mb-6 text-sm">
                                {format!("Etes-vous sur de vouloir supprimer le lien \"{}\" ? Cette action est irreversible.", link_title)}
                            </Text>

                            {move || delete_error.get().map(|err| view! {
                                <div class="mb-6 rounded-lg border border-danger/20 bg-danger/10 p-3">
                                    <Text tone=TextTone::Danger class="text-sm">{err}</Text>
                                </div>
                            })}

                            <div class="flex gap-3">
                                <Button
                                    variant=ButtonVariant::Secondary
                                    size=ButtonSize::Md
                                    full_width=true
                                    on:click=move |_| {
                                        show_modal.set(false);
                                        delete_error.set(None);
                                        deleting_link.set(None);
                                    }
                                >
                                    "Annuler"
                                </Button>
                                <Button
                                    variant=ButtonVariant::Primary
                                    size=ButtonSize::Md
                                    class="bg-danger hover:bg-danger/80 focus-visible:ring-danger"
                                    prop:disabled=move || delete_loading.get()
                                    full_width=true
                                    on:click=move |_| {
                                        let Some(link) = deleting_link.get() else {
                                            return;
                                        };

                                        let Some(token) = auth_store.access_token() else {
                                            delete_error.set(Some("Connexion requise".to_string()));
                                            return;
                                        };

                                        let Some(workspace_id) = workspace_store.selected_id().get() else {
                                            delete_error.set(Some("Aucun workspace selectionne".to_string()));
                                            return;
                                        };

                                        delete_loading.set(true);
                                        delete_error.set(None);

                                        let link_id = link.id.clone();

                                        spawn_local(async move {
                                            match delete_link(&token, &link_id).await {
                                                Ok(_) => {
                                                    delete_loading.set(false);
                                                    show_modal.set(false);
                                                    deleting_link.set(None);

                                                    // Refresh the links list
                                                    links_loading.set(true);
                                                    if let Ok(data) = fetch_links(&workspace_id, &token, current_page.get(), 10).await {
                                                        links.set(data.links);
                                                    }
                                                    links_loading.set(false);
                                                }
                                                Err(err) => {
                                                    delete_error.set(Some(err));
                                                    delete_loading.set(false);
                                                }
                                            }
                                        });
                                    }
                                >
                                    {move || if delete_loading.get() { "Suppression..." } else { "Supprimer" }}
                                </Button>
                            </div>
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
fn delete_confirm_modal(
    _show_modal: RwSignal<bool>,
    _deleting_link: RwSignal<Option<LinkItem>>,
    _delete_loading: RwSignal<bool>,
    _delete_error: RwSignal<Option<String>>,
    _links: RwSignal<Vec<LinkItem>>,
    _links_loading: RwSignal<bool>,
    _current_page: RwSignal<u64>,
) -> impl IntoView {
    view! {}
}

// Links Table Component
#[cfg(feature = "hydrate")]
fn links_table(
    links: RwSignal<Vec<LinkItem>>,
    links_loading: RwSignal<bool>,
    links_error: RwSignal<Option<String>>,
    show_edit_modal: RwSignal<bool>,
    editing_link: RwSignal<Option<LinkItem>>,
    edit_url: RwSignal<String>,
    edit_title: RwSignal<String>,
    edit_is_active: RwSignal<bool>,
    show_delete_modal: RwSignal<bool>,
    deleting_link: RwSignal<Option<LinkItem>>,
    copy_success: RwSignal<Option<String>>,
) -> impl IntoView {
    view! {
        <Card>
            <CardBody class="pt-6">
                <Show
                    when=move || links_loading.get()
                    fallback=move || ()
                >
                    <div class="py-12 text-center">
                        <Text tone=TextTone::Muted>"Chargement des liens..."</Text>
                    </div>
                </Show>

                <Show
                    when=move || links_error.get().is_some()
                    fallback=move || ()
                >
                    {move || links_error.get().map(|err| view! {
                        <div class="rounded-lg border border-danger/20 bg-danger/10 p-4">
                            <Text tone=TextTone::Danger class="text-sm">{err}</Text>
                        </div>
                    })}
                </Show>

                <Show
                    when=move || !links_loading.get() && links_error.get().is_none() && links.get().is_empty()
                    fallback=move || ()
                >
                    <div class="py-12 text-center">
                        <div class="mb-4">
                            <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" class="mx-auto text-foreground/30">
                                <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"></path>
                                <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"></path>
                            </svg>
                        </div>
                        <Heading level=HeadingLevel::H3 class="mb-2">
                            "Aucun lien"
                        </Heading>
                        <Text tone=TextTone::Muted class="text-sm">
                            "Creez votre premier lien raccourci pour commencer"
                        </Text>
                    </div>
                </Show>

                <Show
                    when=move || !links_loading.get() && links_error.get().is_none() && !links.get().is_empty()
                    fallback=move || ()
                >
                    {move || copy_success.get().map(|msg| view! {
                        <div class="mb-4 rounded-lg border border-success/20 bg-success/10 p-3">
                            <Text class="text-sm text-success">{msg}</Text>
                        </div>
                    })}

                    <div class="overflow-x-auto">
                        <table class="w-full">
                            <thead>
                                <tr class="border-b border-border/60">
                                    <th class="py-3 px-4 text-left text-xs font-medium uppercase tracking-wide text-foreground/60">"Titre / URL"</th>
                                    <th class="py-3 px-4 text-left text-xs font-medium uppercase tracking-wide text-foreground/60">"Lien court"</th>
                                    <th class="py-3 px-4 text-left text-xs font-medium uppercase tracking-wide text-foreground/60">"Statut"</th>
                                    <th class="py-3 px-4 text-left text-xs font-medium uppercase tracking-wide text-foreground/60">"Date"</th>
                                    <th class="py-3 px-4 text-right text-xs font-medium uppercase tracking-wide text-foreground/60">"Actions"</th>
                                </tr>
                            </thead>
                            <tbody>
                                <For
                                    each=move || links.get()
                                    key=|link| link.id.clone()
                                    children=move |link| {
                                        let link_for_edit = link.clone();
                                        let link_for_delete = link.clone();
                                        let short_url = link.short_url.clone();
                                        let short_url_for_display = link.short_url.clone();

                                        view! {
                                            <tr class="border-b border-border/30 hover:bg-surface/60 transition-colors">
                                                <td class="py-4 px-4">
                                                    <div class="flex flex-col gap-1">
                                                        <Text class="font-medium text-sm">
                                                            {link.title.clone().unwrap_or_else(|| "Sans titre".to_string())}
                                                        </Text>
                                                        <Text tone=TextTone::Subtle class="text-xs truncate max-w-[300px]">
                                                            {link.original_url.clone()}
                                                        </Text>
                                                    </div>
                                                </td>
                                                <td class="py-4 px-4">
                                                    <Text tone=TextTone::Muted class="font-mono text-sm">
                                                        {short_url_for_display}
                                                    </Text>
                                                </td>
                                                <td class="py-4 px-4">
                                                    <Badge variant={if link.is_active { BadgeVariant::Subtle } else { BadgeVariant::Outline }}>
                                                        {if link.is_active { "Actif" } else { "Inactif" }}
                                                    </Badge>
                                                </td>
                                                <td class="py-4 px-4">
                                                    <Text tone=TextTone::Subtle class="text-sm">
                                                        {link.created_at.clone().map(|d| {
                                                            // Format date simply
                                                            d.split('T').next().unwrap_or(&d).to_string()
                                                        }).unwrap_or_else(|| "-".to_string())}
                                                    </Text>
                                                </td>
                                                <td class="py-4 px-4">
                                                    <div class="flex items-center justify-end gap-2">
                                                        // Copy button
                                                        <button
                                                            class="p-2 rounded-lg text-foreground/60 hover:text-foreground hover:bg-surface transition"
                                                            title="Copier le lien"
                                                            on:click={
                                                                let short_url_for_copy = short_url.clone();
                                                                move |_| {
                                                                    #[cfg(feature = "hydrate")]
                                                                    {
                                                                        use web_sys::js_sys;
                                                                        // Copy to clipboard using JS eval
                                                                        let js_code = format!(
                                                                            "navigator.clipboard.writeText('{}').then(function(){{}}).catch(function(){{}})",
                                                                            short_url_for_copy.replace("'", "\\'")
                                                                        );
                                                                        let _ = js_sys::eval(&js_code);
                                                                        copy_success.set(Some("Lien copie dans le presse-papier!".to_string()));
                                                                        // Note: The success message will stay until user performs another action
                                                                    }
                                                                }
                                                            }
                                                        >
                                                            <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                                                <rect width="14" height="14" x="8" y="8" rx="2" ry="2"></rect>
                                                                <path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"></path>
                                                            </svg>
                                                        </button>

                                                        // Edit button
                                                        <button
                                                            class="p-2 rounded-lg text-foreground/60 hover:text-foreground hover:bg-surface transition"
                                                            title="Modifier"
                                                            on:click=move |_| {
                                                                let link = link_for_edit.clone();
                                                                edit_url.set(link.original_url.clone());
                                                                edit_title.set(link.title.clone().unwrap_or_default());
                                                                edit_is_active.set(link.is_active);
                                                                editing_link.set(Some(link));
                                                                show_edit_modal.set(true);
                                                            }
                                                        >
                                                            <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                                                <path d="M17 3a2.85 2.83 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z"></path>
                                                                <path d="m15 5 4 4"></path>
                                                            </svg>
                                                        </button>

                                                        // Delete button
                                                        <button
                                                            class="p-2 rounded-lg text-danger/60 hover:text-danger hover:bg-danger/10 transition"
                                                            title="Supprimer"
                                                            on:click=move |_| {
                                                                deleting_link.set(Some(link_for_delete.clone()));
                                                                show_delete_modal.set(true);
                                                            }
                                                        >
                                                            <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                                                <path d="M3 6h18"></path>
                                                                <path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"></path>
                                                                <path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"></path>
                                                                <line x1="10" x2="10" y1="11" y2="17"></line>
                                                                <line x1="14" x2="14" y1="11" y2="17"></line>
                                                            </svg>
                                                        </button>
                                                    </div>
                                                </td>
                                            </tr>
                                        }
                                    }
                                />
                            </tbody>
                        </table>
                    </div>
                </Show>
            </CardBody>
        </Card>
    }
}

#[cfg(not(feature = "hydrate"))]
fn links_table(
    _links: RwSignal<Vec<LinkItem>>,
    _links_loading: RwSignal<bool>,
    _links_error: RwSignal<Option<String>>,
    _show_edit_modal: RwSignal<bool>,
    _editing_link: RwSignal<Option<LinkItem>>,
    _edit_url: RwSignal<String>,
    _edit_title: RwSignal<String>,
    _edit_is_active: RwSignal<bool>,
    _show_delete_modal: RwSignal<bool>,
    _deleting_link: RwSignal<Option<LinkItem>>,
    _copy_success: RwSignal<Option<String>>,
) -> impl IntoView {
    view! {
        <Card>
            <CardBody class="pt-6">
                <div class="py-12 text-center">
                    <Text tone=TextTone::Muted>"Chargement..."</Text>
                </div>
            </CardBody>
        </Card>
    }
}

#[component]
pub fn LinksPage() -> impl IntoView {
    #[cfg(feature = "hydrate")]
    let auth_store = use_auth_store();
    #[cfg(feature = "hydrate")]
    let workspace_store = use_workspace_store();

    // Links data
    let links: RwSignal<Vec<LinkItem>> = RwSignal::new(Vec::new());
    let links_loading = RwSignal::new(false);
    let links_error = RwSignal::new(Option::<String>::None);
    let current_page = RwSignal::new(1u64);

    // Create modal state
    let show_create_modal = RwSignal::new(false);
    let form_url = RwSignal::new(String::new());
    let form_title = RwSignal::new(String::new());
    let form_custom_code = RwSignal::new(String::new());
    let form_loading = RwSignal::new(false);
    let form_error = RwSignal::new(Option::<String>::None);

    // Edit modal state
    let show_edit_modal = RwSignal::new(false);
    let editing_link: RwSignal<Option<LinkItem>> = RwSignal::new(None);
    let edit_url = RwSignal::new(String::new());
    let edit_title = RwSignal::new(String::new());
    let edit_is_active = RwSignal::new(true);
    let edit_loading = RwSignal::new(false);
    let edit_error = RwSignal::new(Option::<String>::None);

    // Delete modal state
    let show_delete_modal = RwSignal::new(false);
    let deleting_link: RwSignal<Option<LinkItem>> = RwSignal::new(None);
    let delete_loading = RwSignal::new(false);
    let delete_error = RwSignal::new(Option::<String>::None);

    // Copy success message
    let copy_success = RwSignal::new(Option::<String>::None);

    // Load links when workspace changes
    #[cfg(feature = "hydrate")]
    {
        let selected_workspace = workspace_store.selected_workspace();

        Effect::new(move |_| {
            if let Some(ws) = selected_workspace.get() {
                if let Some(token) = auth_store.access_token() {
                    links_loading.set(true);
                    links_error.set(None);

                    let workspace_id = ws.id.clone();
                    let token_clone = token.clone();
                    let page = current_page.get();

                    spawn_local(async move {
                        match fetch_links(&workspace_id, &token_clone, page, 10).await {
                            Ok(data) => {
                                links.set(data.links);
                                links_loading.set(false);
                            }
                            Err(err) => {
                                links_error.set(Some(err));
                                links_loading.set(false);
                            }
                        }
                    });
                }
            } else {
                links.set(Vec::new());
            }
        });
    }

    #[cfg(feature = "hydrate")]
    let selected_workspace = workspace_store.selected_workspace();
    #[cfg(not(feature = "hydrate"))]
    let selected_workspace = Signal::derive(|| None::<WorkspaceSummary>);

    let has_workspace = Signal::derive(move || selected_workspace.get().is_some());

    view! {
        <DashboardLayout>
            <div class="flex flex-col gap-8">
                <div class="flex items-center justify-between">
                    <div class="flex flex-col gap-2">
                        <Heading level=HeadingLevel::H1>
                            "Liens"
                        </Heading>
                        <Text tone=TextTone::Muted>
                            "Gerez tous vos liens raccourcis"
                        </Text>
                    </div>
                    <Show
                        when=move || has_workspace.get()
                        fallback=move || ()
                    >
                        <Button
                            variant=ButtonVariant::Primary
                            on:click=move |_| show_create_modal.set(true)
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <path d="M5 12h14"></path>
                                <path d="M12 5v14"></path>
                            </svg>
                            "Nouveau lien"
                        </Button>
                    </Show>
                </div>

                <Show
                    when=move || !has_workspace.get()
                    fallback=move || ()
                >
                    <Card>
                        <CardBody class="pt-6">
                            <div class="flex flex-col items-center justify-center gap-6 py-12 text-center">
                                <div class="flex flex-col gap-2">
                                    <Heading level=HeadingLevel::H2 class="text-xl">
                                        "Aucun workspace selectionne"
                                    </Heading>
                                    <Text tone=TextTone::Muted class="text-sm">
                                        "Selectionnez un workspace existant ou creez-en un nouveau pour gerer vos liens"
                                    </Text>
                                </div>
                            </div>
                        </CardBody>
                    </Card>
                </Show>

                <Show
                    when=move || has_workspace.get()
                    fallback=move || ()
                >
                    {links_table(
                        links,
                        links_loading,
                        links_error,
                        show_edit_modal,
                        editing_link,
                        edit_url,
                        edit_title,
                        edit_is_active,
                        show_delete_modal,
                        deleting_link,
                        copy_success,
                    )}
                </Show>
            </div>

            // Modals
            {create_link_modal(
                show_create_modal,
                form_url,
                form_title,
                form_custom_code,
                form_loading,
                form_error,
                links,
                links_loading,
                current_page,
            )}
            {edit_link_modal(
                show_edit_modal,
                editing_link,
                edit_url,
                edit_title,
                edit_is_active,
                edit_loading,
                edit_error,
                links,
                links_loading,
                current_page,
            )}
            {delete_confirm_modal(
                show_delete_modal,
                deleting_link,
                delete_loading,
                delete_error,
                links,
                links_loading,
                current_page,
            )}
        </DashboardLayout>
    }
}
