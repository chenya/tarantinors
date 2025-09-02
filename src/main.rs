mod movies;

use axum::{response::Html, routing::get, Json, Router};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(root))
        .route("/hello", get(hello_msg));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

    tracing::info!("Listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener,app).await.unwrap();

}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Msg {
    message: String,
}

#[tracing::instrument]
async fn root() -> Html<&'static str> {
    tracing::info!("Sending hello world response");
    Html("<h1>Hello, World!</h1>")
}

#[tracing::instrument]
async fn hello_msg() -> Json<Msg> {
    tracing::info!("Sending json hello world response");

    Json(Msg{
        message: "Hello, World!".to_string(),
    })
}