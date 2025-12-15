use sqlx::postgres::{PgPool, PgPoolOptions};

use crate::movies::data::store::MoviesStore;

pub async fn init_dbpool() -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:1234@localhost:7777/tarantinodb".to_string());

    let db_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_connection_str)
        .await;

    db_pool
}

#[derive(Clone, Debug)]
pub struct Store {
    pub connection: PgPool,
    pub movies_store: MoviesStore,
}

impl Store {
    pub async fn new() -> Self {
        let connection = match init_dbpool().await {
            Ok(pool) => pool,
            Err(e) => {
                panic!("Failed to create database pool: {}", e);
            }
        };
        let movies_store = MoviesStore::new(connection.clone());
        Self {
            connection,
            movies_store,
        }
    }
}
