#![recursion_limit = "512"]

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{
        extract::Extension,
        routing::{delete, get, post, put},
        Router,
    };
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use shrtnr::backend;
    use shrtnr::backend::auth::handlers::{
        login_handler, logout_handler, me_handler, refresh_token_handler, register_handler,
        update_profile_handler,
    };
    use shrtnr::backend::links::handlers::{
        create_link_handler, delete_link_handler, get_link_handler, list_links_handler,
        redirect_handler, update_link_handler,
    };
    use shrtnr::backend::stats::handlers::{
        get_dashboard_stats_handler, get_link_stats_handler, get_workspace_stats_handler,
    };
    use shrtnr::backend::workspaces::handlers::{
        add_workspace_member_handler, create_workspace_handler, delete_workspace_handler,
        get_workspace_handler, list_workspaces_handler, remove_workspace_member_handler,
        update_workspace_handler,
    };
    use shrtnr::frontend::{shell, App};

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    // Initialize database connection (supports Postgres or SQLite via DATABASE_URL)
    let db = backend::connect()
        .await
        .expect("failed to connect to database");

    // Get JWT secret from environment or use default for development
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "your-secret-key-change-this-in-production".to_string());

    // Create application state
    let app_state = backend::AppState::new(db.clone(), &jwt_secret);

    // First, create the Leptos router with its state
    let leptos_router = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // Create API router with AppState extensions
    // This router uses layers for state instead of .with_state()
    let api_router = Router::new()
        // API Auth routes
        .route("/api/auth/register", post(register_handler))
        .route("/api/auth/login", post(login_handler))
        .route("/api/auth/logout", post(logout_handler))
        .route("/api/auth/refresh-token", post(refresh_token_handler))
        .route("/api/auth/me", get(me_handler))
        .route("/api/auth/profile", put(update_profile_handler))
        // API Link management routes
        .route("/api/links", post(create_link_handler))
        .route("/api/links", get(list_links_handler))
        .route("/api/links/{id}", get(get_link_handler))
        .route("/api/links/{id}", put(update_link_handler))
        .route("/api/links/{id}", delete(delete_link_handler))
        // API Workspace routes
        .route("/api/workspaces", post(create_workspace_handler))
        .route("/api/workspaces", get(list_workspaces_handler))
        .route("/api/workspaces/{id}", get(get_workspace_handler))
        .route("/api/workspaces/{id}", put(update_workspace_handler))
        .route("/api/workspaces/{id}", delete(delete_workspace_handler))
        .route(
            "/api/workspaces/{id}/members",
            post(add_workspace_member_handler),
        )
        .route(
            "/api/workspaces/{id}/members/{userId}",
            delete(remove_workspace_member_handler),
        )
        // API Stats routes
        .route("/api/links/{id}/stats", get(get_link_stats_handler))
        .route(
            "/api/workspaces/{id}/stats",
            get(get_workspace_stats_handler),
        )
        .route("/api/stats/dashboard", get(get_dashboard_stats_handler))
        // Public redirect route (must be before leptos_routes to catch short codes)
        .route("/{short_code}", get(redirect_handler))
        .layer(Extension(db))
        .layer(Extension(app_state));

    // Merge the two routers
    let app = Router::new().merge(api_router).merge(leptos_router);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    let app_service = app.into_make_service_with_connect_info::<std::net::SocketAddr>();
    axum::serve(listener, app_service).await.unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
