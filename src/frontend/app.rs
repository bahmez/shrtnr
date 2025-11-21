use crate::frontend::design_system::{
    Badge, BadgeVariant, Button, ButtonSize, ButtonVariant, Card, CardBody, CardFooter, CardHeader,
    Chip, FormControl, Heading, HeadingLevel, InputField, Text, TextTone,
};
use leptos::ev;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body class="min-h-screen bg-background text-foreground antialiased selection:bg-brand/80 selection:text-brand-foreground">
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/shrtnr.css"/>

        // sets the document title
        <Title text="shrtnr · Design system Tailwind"/>

        // content for this welcome page
        <Router>
            <main class="relative">
                <Routes fallback=|| {
                    view! { <div class="p-6 text-sm text-danger">"Page introuvable"</div> }.into_view()
                }>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let workspace_name = RwSignal::new("Campagne Print 2025".to_string());
    let auto_refresh = RwSignal::new(true);

    let on_name_input = {
        let workspace_name = workspace_name.clone();
        Callback::new(move |ev: ev::Event| {
            workspace_name.set(event_target_value(&ev));
        })
    };

    let toggle_refresh = {
        let auto_refresh = auto_refresh.clone();
        Callback::new(move |_| {
            auto_refresh.update(|value| *value = !*value);
        })
    };

    let preview_url = Memo::new(move |_| {
        let mut slug = workspace_name
            .get()
            .trim()
            .to_lowercase()
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == ' ')
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join("-");

        if slug.is_empty() {
            slug = "votre-lien".to_string();
        }

        format!("shrtnr.dev/{slug}")
    });

    view! {
        <div class="mx-auto flex min-h-[calc(100vh-6rem)] w-full max-w-6xl flex-col gap-12 px-6 py-16 lg:px-12">
            <section class="grid gap-6 text-left">
                <Badge variant=BadgeVariant::Subtle>"Design System"</Badge>
                <Heading level=HeadingLevel::H1 class="max-w-3xl">
                    "Une base UI cohérente prête pour vos produits."
                </Heading>
                <Text tone=TextTone::Muted class="max-w-2xl">
                    "Composez rapidement vos écrans grâce à des primitives Leptos typées, construites au-dessus d'une configuration Tailwind unifiée."
                </Text>
                <div class="mt-4 flex flex-wrap items-center gap-3">
                    <Button variant=ButtonVariant::Primary size=ButtonSize::Lg>
                        "Créer un nouveau projet"
                    </Button>
                    <Button variant=ButtonVariant::Outline size=ButtonSize::Lg>
                        "Explorer la documentation"
                    </Button>
                    <Chip
                        class="bg-surface-strong/80 backdrop-blur"
                        leading_icon=view! { <span class="text-brand">"●"</span> }.into_any()
                    >
                        {move || if auto_refresh.get() { "Auto-sync activé" } else { "Auto-sync désactivé" }}
                    </Chip>
                </div>
            </section>

            <section class="grid gap-6 lg:grid-cols-2">
                <Card>
                    <CardHeader>
                        <Heading level=HeadingLevel::H3>"Configuration rapide"</Heading>
                        <Text tone=TextTone::Muted>
                            "Définissez le nom de votre workspace et partagez un lien cohérent avec votre équipe."
                        </Text>
                    </CardHeader>
                    <CardBody class="flex flex-col gap-6">
                        <FormControl
                            label="Nom du workspace"
                            hint="Utilisé pour générer automatiquement un slug partageable."
                        >
                            <InputField
                                id="workspace-name"
                                name="workspace_name"
                                placeholder="Marketing Q1"
                                value=workspace_name.get()
                                on_input=on_name_input
                            />
                        </FormControl>

                        <div class="flex items-center justify-between rounded-xl border border-dashed border-border/60 bg-surface px-4 py-3 font-mono text-xs text-foreground/80 sm:text-sm">
                            <span>{move || preview_url.get()}</span>
                            <Button variant=ButtonVariant::Ghost size=ButtonSize::Sm>
                                "Copier"
                            </Button>
                        </div>
                    </CardBody>
                    <CardFooter class="justify-between">
                        <Text tone=TextTone::Subtle class="text-xs sm:text-sm">
                            "Les variantes Tailwind sont appliquées automatiquement selon le thème global."
                        </Text>
                        <Button
                            variant=ButtonVariant::Secondary
                            size=ButtonSize::Sm
                            on_click=toggle_refresh
                        >
                            {move || if auto_refresh.get() { "Désactiver l'auto-sync" } else { "Activer l'auto-sync" }}
                        </Button>
                    </CardFooter>
                </Card>

                <Card class="bg-gradient-to-br from-surface to-surface-strong/70">
                    <CardHeader>
                        <Heading level=HeadingLevel::H3>"Primitives disponibles"</Heading>
                        <Text tone=TextTone::Muted>
                            "Des composants accessibles que vous combinez pour créer votre design system produit."
                        </Text>
                    </CardHeader>
                    <CardBody class="flex flex-col gap-5">
                        <div class="flex flex-wrap gap-2">
                            <Badge variant=BadgeVariant::Solid>"Badge"</Badge>
                            <Badge variant=BadgeVariant::Subtle>"Chip"</Badge>
                            <Badge variant=BadgeVariant::Outline>"Card"</Badge>
                        </div>
                        <Text>
                            "Chaque primitive expose des variantes et des états pour couvrir les cas usuels tout en restant simple à étendre."
                        </Text>
                        <div class="grid gap-3">
                            <Button variant=ButtonVariant::Primary size=ButtonSize::Md full_width=true>
                                "Bouton pleine largeur"
                            </Button>
                            <Button variant=ButtonVariant::Outline size=ButtonSize::Md>
                                "Bouton secondaire"
                            </Button>
                        </div>
                    </CardBody>
                    <CardFooter class="justify-end">
                        <Text tone=TextTone::Subtle class="text-xs">
                            "Ajoutez vos propres primitives dans `frontend/design_system` pour compléter la bibliothèque."
                        </Text>
                    </CardFooter>
                </Card>
            </section>
        </div>
    }
}
