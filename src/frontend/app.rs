//! Point d'entrée de l'application frontend Leptos.
//!
//! Ce module définit la structure HTML de base et configure le router
//! pour toutes les pages de l'application.

use crate::frontend::pages::{
    AidePage, AnalyticsPage, LandingPage, LayoutTestPage, LinksPage, LoginPage, RegisterPage,
    SettingsPage, StatusPage, SupportPage, WorkspacePage,
};
use crate::frontend::state::{provide_auth_store, provide_workspace_store};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

/// Fonction shell qui génère le HTML de base de l'application.
///
/// Cette fonction est utilisée par Leptos pour générer le document HTML
/// lors du rendu côté serveur. Elle inclut :
/// - Les meta tags et le viewport
/// - Les scripts d'hydratation
/// - Le composant App principal
///
/// # Arguments
///
/// * `options` - Options de configuration Leptos (adresse, features, etc.)
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

/// Composant racine de l'application.
///
/// Ce composant :
/// - Configure le contexte meta (stylesheets, titles, etc.)
/// - Initialise les stores d'état (auth, workspaces)
/// - Configure le router avec toutes les routes de l'application
///
/// ## Routes disponibles
///
/// - `/` : Page d'accueil (landing page)
/// - `/login` : Page de connexion
/// - `/register` : Page d'inscription
/// - `/workspace` : Gestion des workspaces
/// - `/links` : Gestion des liens raccourcis
/// - `/analytics` : Analytics et statistiques
/// - `/settings` : Paramètres utilisateur
/// - `/support`, `/status`, `/aide` : Pages d'information
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let auth_store = provide_auth_store();
    provide_workspace_store(auth_store);

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/shrtnr.css"/>

        // sets the document title
        <Title text="shrtnr · Raccourcisseur d'URL intelligent"/>

        // content for this welcome page
        <Router>
            <main class="relative">
                <Routes fallback=|| {
                    view! { <div class="p-6 text-sm text-danger">"Page introuvable"</div> }.into_view()
                }>
                    <Route path=StaticSegment("") view=LandingPage/>
                    <Route path=StaticSegment("layout_test") view=LayoutTestPage/>
                    <Route path=StaticSegment("login") view=LoginPage/>
                    <Route path=StaticSegment("register") view=RegisterPage/>
                    <Route path=StaticSegment("workspace") view=WorkspacePage/>
                    <Route path=StaticSegment("links") view=LinksPage/>
                    <Route path=StaticSegment("analytics") view=AnalyticsPage/>
                    <Route path=StaticSegment("settings") view=SettingsPage/>
                    <Route path=StaticSegment("support") view=SupportPage/>
                    <Route path=StaticSegment("status") view=StatusPage/>
                    <Route path=StaticSegment("aide") view=AidePage/>
                </Routes>
            </main>
        </Router>
    }
}
