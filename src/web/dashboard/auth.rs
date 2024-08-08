use axum::{
    extract,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::Deserialize;

use crate::serve_page;

pub fn setup() -> Router {
    let app = Router::new()
        .route("/login", post(login))
        .route("/login", get(serve_page!("/auth/login")));

    app
}

#[derive(Deserialize)]
struct LoginUser {
    email: String,
    password: String,
}

async fn login(extract::Json(payload): extract::Json<LoginUser>) -> impl IntoResponse {
    (StatusCode::OK, "Payload Received")
}
