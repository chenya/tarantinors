use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new().route("/", get(root));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

    tracing::info!("Listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener,app).await.unwrap();

}

#[tracing::instrument]
async fn root() -> Html<&'static str> {
    tracing::info!("Sending hello world response");
    Html("<h1>Hello, World!</h1>")
}