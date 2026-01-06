use axum::Json;
use axum_test::{TestServer, TestServerBuilder, TestServerConfig};
use serde_json::{Value, json};
use sqlx::{Pool, Postgres};
// use uuid::Uuid;
use super::rest_api_router as router;
use super::store;
use crate::store::init_dbpool;
use axum::http::StatusCode;
use serial_test::serial;

pub async fn create_test_server() -> TestServer {
    // Initialize test server with clean state
    let pool = init_dbpool().await.unwrap();
    let app = router(store::MoviesStore::new(pool.clone()));
    let config = TestServerBuilder::new()
        // .save_cookies()
        .default_content_type(&"application/json")
        .http_transport_with_ip_port(None, Some(3000))
        .into_config();

    TestServer::new_with_config(app, config).unwrap()
}

// pub async fn reset_db(pool: &Pool<Postgres>) {
//     // Reset the database to a clean state before each test
//     sqlx::query("TRUNCATE TABLE movies RESTART IDENTITY CASCADE")
//         .execute(pool)
//         .await
//         .unwrap();
// }

// pub async run_migrations(pool: &Pool<Postgres>) {
//     // Run migrations to set up the database schema
//     sqlx::migrate!("./migrations")
//         .run(pool)
//         .await
//         .unwrap();
//
// }

#[tokio::test]
#[serial]
async fn test_get_movies_list_success() {
    let server = create_test_server().await;

    let response = server.get("/api").await;

    assert_eq!(response.status_code(), StatusCode::OK);
}

#[tokio::test]
#[serial]
async fn test_get_single_movie_with_id_success() {
    let server = create_test_server().await;

    let response = server.get("/api/1").await;

    assert_eq!(response.status_code(), StatusCode::OK);
}

#[tokio::test]
#[serial]
async fn test_get_single_movie_with_id_not_found() {
    let server = create_test_server().await;

    let response = server.get("/api/9876").await;

    assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
}
