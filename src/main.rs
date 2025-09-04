mod movies;
mod store;

use axum::{Json, Router, response::Html, routing::{get, post}};
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use std::str::FromStr;
use axum::extract::State;
use tracing::{info, instrument};
use tracing_subscriber::{EnvFilter, filter::LevelFilter, fmt, prelude::*};
use movies::models::MovieInput;
use store::Store;
fn init_tracing() {
    let rust_log = std::env::var(EnvFilter::DEFAULT_ENV)
        .unwrap_or_else(|_| "sqlx=info,tower_http=debug,info".to_string());
    tracing_subscriber::registry()
        // .with(fmt::layer())
        .with(
            fmt::layer()
                .with_target(true)
                .with_line_number(true)
                .with_file(true)
                .pretty(),
        )
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .parse_lossy(rust_log),
        )
        .init();
}

#[derive(Clone, Debug)]
struct AppState {
    store: Store,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app_state = AppState {
        store: Store::new().await,
    };

    let app = Router::new()
        .route("/", get(root))
        .route("/hello", get(hello_msg))
        .route("/movies", post(add_movie))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tracing::info!("Listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[instrument]
async fn add_movie(State(state): State<AppState>, Json(new_movie): Json<MovieInput>) -> Json<Msg> {
    state.store.insert_movie(new_movie).await.unwrap();
    tracing::info!("Adding a movie");
    Json(Msg {
        message: "Movie added".to_string(),
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Msg {
    message: String,
}

#[instrument]
async fn root() -> Html<&'static str> {
    tracing::info!("Sending hello world response");
    Html("<h1>Hello, World!</h1>")
}

#[instrument]
async fn hello_msg() -> Json<Msg> {
    tracing::info!("Sending json hello world response");

    Json(Msg {
        message: "Hello, World!".to_string(),
    })
}
