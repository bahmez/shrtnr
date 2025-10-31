
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{extract::{FromRef, Extension}, Router, routing::{get, post, put}};
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use shrtnr::app::*;
    use shrtnr::db;
    use shrtnr::config::AppState;
    use shrtnr::auth::handlers::{
        register_handler,
        login_handler,
        logout_handler,
        refresh_token_handler,
        me_handler,
        update_profile_handler,
    };

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    // Initialize database connection (supports Postgres or SQLite via DATABASE_URL)
    let db = db::connect().await.expect("failed to connect to database");

    // Get JWT secret from environment or use default for development
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "your-secret-key-change-this-in-production".to_string());

    // Create application state
    let app_state = AppState::new(db.clone(), &jwt_secret);

    // Create API routes
    let api_routes = Router::new()
        .route("/auth/register", post(register_handler))
        .route("/auth/login", post(login_handler))
        .route("/auth/logout", post(logout_handler))
        .route("/auth/refresh-token", post(refresh_token_handler))
        .route("/auth/me", get(me_handler))
        .route("/auth/profile", put(update_profile_handler))
        .with_state(app_state.clone());

    let app = Router::new()
        .nest("/api", api_routes)
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .layer(Extension(db))
        .layer(Extension(app_state))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    let app_service = app.into_make_service();
    axum::serve(listener, app_service)
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
