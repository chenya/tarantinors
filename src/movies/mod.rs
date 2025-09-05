pub mod db;
pub mod handlers;
pub mod models;
pub mod store;

use crate::movies::store::MoviesStore;
use axum::{
    Extension, Router,
    routing::{delete, get, post},
};

pub fn router(movie_store: MoviesStore) -> Router {
    let router = Router::new()
        .route("/api", get(handlers::get_movies))
        .route("/api", post(handlers::add_movie))
        .route("/api/{movie_id}", get(handlers::get_movie))
        .route("/api/{movie_id}/actors", get(handlers::get_movie_actors))
        .route(
            "/api/{movie_id}/directors",
            get(handlers::get_movie_directors),
        )
        .route(
            "/api/{movie_id}/producers",
            get(handlers::get_movie_producers),
        )
        .route("/api/{movie_id}/writers", get(handlers::get_movie_writers))
        .route("/api/{movie_id}/awards", get(handlers::get_movie_awards))
        .route(
            "/api/{movie_id}/nominations",
            get(handlers::get_movie_nominations),
        )
        .route("/api/{movie_id}", delete(handlers::remove_movie))
        .layer(Extension(movie_store));

    router
}
