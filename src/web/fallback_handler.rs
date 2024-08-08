use axum::{
    http::{StatusCode, Uri},
    response::IntoResponse,
    Router,
};

pub fn add_fallback(app: Router) -> Router {
    return app.fallback(handler_404);
}

async fn handler_404(uri: Uri) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}
