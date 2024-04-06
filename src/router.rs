use crate::middleware::{print_request_body, BufferRequestBody};
use axum::{
    http::{HeaderMap, Method, Uri},
    middleware,
    routing::{delete, get, head, options, patch, post},
    Router,
};
use nu_ansi_term::Color::Green;

/// Create the router
pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/", get(handler))
        .route("/*W", get(handler))
        .route("/", post(handler))
        .route("/*W", post(handler))
        .route("/", patch(handler))
        .route("/*W", patch(handler))
        .route("/", delete(handler))
        .route("/*W", delete(handler))
        .route("/", options(handler))
        .route("/*W", options(handler))
        .route("/", head(handler))
        .route("/*W", head(handler))
        .layer(middleware::from_fn(print_request_body))
        .route(
            "/favicon.ico",
            get(|| async move { "Favicon request ignored" }),
        )
}

/// Request handler
async fn handler(
    uri: Uri,
    method: Method,
    headers: HeaderMap,
    BufferRequestBody(body): BufferRequestBody,
) -> &'static str {
    let method_str = Green.bold().paint(format!("{:?}", method));
    let headers_str = Green.bold().paint("Headers:");
    let body_str = Green.bold().paint("Body:");

    tracing::info!(
        "\n{} {}\n{}\n{:?}\n{}\n{:?}",
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
