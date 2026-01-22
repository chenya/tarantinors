mod docs;
mod interviews;
mod movies;
mod quotes;
mod store;

// mod tests;
use axum::extract::Path;

use axum::body::Body;
use axum::http::Request;
use axum::{response::Html, routing::get, Json, Router};

use serde::{Deserialize, Serialize};

use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::{info, instrument};
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::{filter::LevelFilter, fmt, prelude::*, EnvFilter};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use askama::Template;
use store::Store;

fn init_tracing() {
    let rust_log = std::env::var(EnvFilter::DEFAULT_ENV)
        .unwrap_or_else(|_| "sqlx=info,tower_http=debug,debug".to_string());

    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_target(false)
                .with_line_number(true)
                .with_file(true)
                .with_span_events(FmtSpan::CLOSE)
                .pretty(),
        )
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .parse_lossy(rust_log),
        )
        .init();
}

fn make_app_router(db_store: &Store) -> Router {
    let movies_api_router = movies::rest_api_router(db_store);
    let movies_web_router = movies::web_router(db_store);
    let movies_htmx_web_router = movies::htmx_web_router(db_store);
    let quotes_api_router = quotes::rest_api_router(db_store);
    let quotes_web_router = quotes::web_router(db_store);
    let quotes_htmx_web_router = quotes::htmx_web_router(db_store);
    let interviews_api_router = interviews::rest_api_router(db_store);
    let interviews_web_router = interviews::web_router(db_store);
    let interviews_htmx_web_router = interviews::htmx_web_router(db_store);

    let app_router = Router::new()
        .route("/", get(home))
        .nest("/movies", movies_web_router)
        .nest("/quotes", quotes_web_router)
        .nest("/interviews", interviews_web_router)
        .route("/htmx", get(htmx_home))
        .nest("/htmx/movies", movies_htmx_web_router)
        .nest("/htmx/quotes", quotes_htmx_web_router)
        .nest("/htmx/interviews", interviews_htmx_web_router)
        .nest("/api/v1", movies_api_router)
        .nest("/api/v1", quotes_api_router)
        .nest("/api/v1", interviews_api_router)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", docs::ApiDoc::openapi()))
        .nest_service("/static", ServeDir::new("static"))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                let request_id = uuid::Uuid::new_v4();
                tracing::span!(
                    tracing::Level::INFO,
                    "request",
                    method = display(request.method()),
                    uri = display(request.uri()),
                    version = debug(request.version()),
                    request_id = display(request_id)
                )
            }),
        );
    app_router
}
#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();

    init_tracing();

    let db_store = Store::new().await;

    let app = make_app_router(&db_store);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    info!("Listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
    info!("Shutdown signal received");
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Msg {
    message: String,
}

#[derive(Template)]
#[template(path = "pages/home.html")]
pub struct HomeTemplate;

#[instrument]
async fn home() -> Html<String> {
    let home_template = HomeTemplate {}.render().unwrap();
    info!("Welcome to Quentin Tarantino home page");
    Html(home_template)
}

#[derive(Template)]
#[template(path = "pages/htmx/home.html")]
pub struct HtmxHomeTemplate;

#[instrument]
async fn htmx_home() -> Html<String> {
    let htmx_home_template = HtmxHomeTemplate {}.render().unwrap();
    info!("Welcome to Quentin Tarantino htmx home page");
    Html(htmx_home_template)
}

#[instrument]
async fn hello_msg() -> Json<Msg> {
    tracing::info!("Sending json hello world response");

    Json(Msg {
        message: "Hello, World!".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::response::IntoResponse;
    use http_body_util::BodyExt;
    // for `oneshot`

    #[tokio::test]
    async fn test_hello_msg() {
        let response = hello_msg().await.into_response();

        assert_eq!(response.status(), 200);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = String::from_utf8(body.to_vec()).unwrap();

        assert!(body_str.contains("Hello, World!"));
    }
}
