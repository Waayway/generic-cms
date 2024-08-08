use axum::{Router, ServiceExt};
use tower::Layer;

use crate::{config::Config, utils::trailing_slash, AppState};

mod dashboard;
mod graphql;

mod fallback_handler;

pub async fn init_webserver(state: AppState) {
    //
    //  NOTE: Build our application with base routing
    //
    let mut app = Router::new()
        .with_state(state)
        .merge(dashboard::setup())
        .nest("/graphql", graphql::setup());

    //
    //  NOTE: Add fallback handler to the app
    //
    app = fallback_handler::add_fallback(app);

    //
    //  NOTE: Remove trailing slashes for routes. see utils/trailing_slash.rs for more info
    //
    let middleware = tower::util::MapRequestLayer::new(trailing_slash::rewrite_request_uri);

    //
    // Apply middleware
    //
    let app = middleware.layer(app);

    //
    //  NOTE: Fetch environment config for app url and port
    //
    let config = Config::get_env();

    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        config.app_url.unwrap_or("127.0.0.1".to_string()),
        config.app_port.unwrap_or(3000)
    ))
    .await
    .unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
