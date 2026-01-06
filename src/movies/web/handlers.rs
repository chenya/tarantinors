use crate::movies::{
    data::store::MoviesStore,
    web::{
        errors::MoviesWebError,
        models::MovieViewModel,
        service::WebService,
        templates::{BaseTemplate, HomeTemplate, MovieDetailsTemplate, MoviesTemplate},
    },
};

use askama::Template;
use axum::{Extension, extract::Path, http::StatusCode, response::Html};
use tracing::{info, instrument};

#[instrument]
pub async fn home() -> Html<String> {
    let home_template = HomeTemplate {}.render().unwrap();

    info!("queried all movies");
    Html(home_template)
}

#[instrument]
pub async fn movie_details(
    Extension(store): Extension<MoviesStore>,
    Path(movie_id): Path<i32>,
) -> Result<Html<String>, MoviesWebError> {
    let service = WebService::new(&store.connection);

    let movie = service
        .get_movie(movie_id)
        .await?
        .ok_or(|e| MoviesWebError::NotFound(movie_id))?;

    let base_template = MovieDetailsTemplate { movie }.render().unwrap();

    info!("queried movie {}", movie_id);
    Ok(Html(base_template))
}

#[instrument]
pub async fn list_movies(Extension(store): Extension<MoviesStore>) -> Html<String> {
    let service = WebService::new(&store.connection);

    let movies = service.get_movies().await.unwrap();

    let movies_template = MoviesTemplate { movies }.render().unwrap();

    info!("queried all movies");
    Html(movies_template)
}
