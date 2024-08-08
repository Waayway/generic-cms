use axum::{
    http::{header, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use rust_embed::Embed;

pub fn serve_router() -> Router {
    let router = Router::new()
        .route("/", get(index_handler))
        .route("/*file", get(static_handler));

    router
}

async fn index_handler() -> impl IntoResponse {
    static_handler("/index.html".parse::<Uri>().unwrap()).await
}

#[macro_export]
macro_rules! serve_page {
    ($route:expr) => {
        || async { super::serve::static_handler($route.parse::<axum::http::Uri>().unwrap()).await }
    };
}

pub async fn static_handler(uri: Uri) -> impl IntoResponse {
    //tracing::debug!("Fetching file for uri: {uri}");
    let mut path = uri.path().trim_start_matches("/").to_string();

    if path.starts_with("dist/") {
        path = path.replace("dist/", "")
    }

    StaticFile(path)
}

#[derive(Embed)]
#[folder = "src/web/dashboard/dist"]
struct Asset;

pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path: String = self.0.into();

        match Asset::get(path.as_str()) {
            Some(content) => {
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
            }
            None => match path.ends_with("index.html") {
                true => (StatusCode::NOT_FOUND, "404 Not found").into_response(),
                false => match path.ends_with("/") {
                    true => StaticFile(format!("{}index.html", path)).into_response(),
                    false => StaticFile(format!("{}/index.html", path)).into_response(),
                },
            },
        }
    }
}
