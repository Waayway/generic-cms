use axum::http::{Request, Uri};
use tracing::info;

pub fn rewrite_request_uri<B>(req: Request<B>) -> Request<B> {
    let mut req = req;
    let uri: String = req.uri().to_string();

    info!("Got Request: {}", uri);

    *req.uri_mut() = uri.trim_end_matches("/").parse().unwrap_or(Uri::default());

    req
}
