use crate::movies::models::{
    CreateAward, CreateGenre, CreateMovie, CreatePerson, MovieInput, Person,
};

use serde::Deserialize;
use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};

use crate::movies::db::{
    create_actor, create_award, create_award_category, create_director, create_genre, create_movie,
    create_movie_award, create_movie_genre, create_movie_nomination, create_person,
    create_producer, create_writer,
};

use crate::movies::store::MoviesStore;

async fn init_dbpool() -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
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
