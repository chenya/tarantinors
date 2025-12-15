mod movies;
mod store;

use axum::extract::Path;
use movies::{api::handlers::MoviesApiDoc, data::store::MoviesStore};

use crate::store::init_dbpool;
// use movies::data::store::MoviesStore;

use axum::body::Body;
use axum::http::Request;
use axum::{Json, Router, response::Html, routing::get};

use serde::{Deserialize, Serialize};

use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::{info, instrument};
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::{EnvFilter, filter::LevelFilter, fmt, prelude::*};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use askama::Template;

#[derive(Template)]
#[template(path = "hello_name.html")]
pub struct HelloName {
    pub name: String,
}

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

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Movies  API",
        description = "A comprehensive Movies API with validation",
        version = "1.0.0",
        contact(
            name = "API Support",
            email = "support@example.com"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    servers(
        (url = "http://localhost:3000", description = "Development server"),
    ),
    nest(
        (path = "/api/v1", api = MoviesApiDoc)
    ),
)]
pub struct ApiDoc;

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();

    init_tracing();
    let dbpool = init_dbpool().await.unwrap();

    let movies_api_router = movies::rest_api_router(MoviesStore {
        connection: dbpool.clone(),
    });

    let app = Router::new()
        .route("/", get(root))
        .route("/hello", get(hello_msg))
        .route("/hello/{name}", get(hello_name))
        .nest("/api/v1", movies_api_router)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
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

#[instrument]
async fn root() -> Html<&'static str> {
    info!("Sending hello world response");
    Html("<h1>Hello, World!</h1>")
}

#[instrument]
async fn hello_name(Path(name): Path<String>) -> Html<String> {
    info!("Sending hello world response to {}", name);
    let template = HelloName { name: name };
    Html(template.render().unwrap())
}

#[instrument]
async fn hello_msg() -> Json<Msg> {
    tracing::info!("Sending json hello world response");

    Json(Msg {
        message: "Hello, World!".to_string(),
    })
}
