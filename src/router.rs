use axum::{
    http::{HeaderMap, Method, Uri},
    routing::{delete, get, head, options, patch, post},
    Router,
};
use nu_ansi_term::Color::Green;

/// Create the router
pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/*W", post(handler))
        .route("/*W", get(handler))
        .route("/*W", patch(|| async move { "PATCH /" }))
        .route("/*W", delete(|| async move { "DELETE /" }))
        .route("/*W", options(|| async move { "OPTIONS /" }))
        .route("/*W", head(|| async move { "HEAD /" }))
        .route(
            "/favicon.ico",
            get(|| async move { "Favicon request ignored" }),
        )
}

/// Request handler
async fn handler(uri: Uri, method: Method, headers: HeaderMap, body: String) -> &'static str {
    let method_str = Green.bold().paint(format!("{:?}", method));
    let headers_str = Green.bold().paint("Headers:");
    let body_str = Green.bold().paint("Body:");

    tracing::info!(
        "\n{} {}\n{}\n{:?}\n{}\n{}",
        method_str,
        uri,
        headers_str,
        headers,
        body_str,
        body
    );

    "OK"
}

/// Health check endpoint
async fn health() -> &'static str {
    "OK"
}
