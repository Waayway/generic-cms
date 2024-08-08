use axum::Router;

mod auth;
mod serve;

pub fn setup() -> Router {
    let app = Router::new()
        .merge(serve::serve_router())
        .nest("/auth", auth::setup());

    app
}
