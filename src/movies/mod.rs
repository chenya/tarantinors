pub mod api;
pub mod data;
pub mod web;

use axum::{
    Extension, Router,
    routing::{delete, get, post},
};
use data::store::MoviesStore;

pub fn rest_api_router(movie_store: MoviesStore) -> Router {
    let router = Router::new()
        .route("/movies", get(api::handlers::get_movies))
        .route("/movies", post(api::handlers::add_movie))
        .route("/movies/{movie_id}", get(api::handlers::get_movie))
        .route(
            "/movies/{movie_id}/actors",
            get(api::handlers::get_movie_actors),
        )
        .route(
            "/movies/{movie_id}/directors",
            get(api::handlers::get_movie_directors),
        )
        .route(
            "/movies/{movie_id}/producers",
            get(api::handlers::get_movie_producers),
        )
        .route(
            "/movies/{movie_id}/writers",
            get(api::handlers::get_movie_writers),
        )
        .route(
            "/movies/{movie_id}/awards",
            get(api::handlers::get_movie_awards),
        )
        .route(
            "/movies/{movie_id}/nominations",
            get(api::handlers::get_movie_nominations),
        )
        .route("/movies/{movie_id}", delete(api::handlers::remove_movie))
        .fallback(api::handlers::fallback_handler)
        .layer(Extension(movie_store));

    router
}

pub fn web_router(movie_store: MoviesStore) -> Router {
    let router = Router::new()
        .route("/", get(web::handlers::list_movies))
        .route("/{movie_id}", get(web::handlers::movie_details))
        .layer(Extension(movie_store));

    router
}
