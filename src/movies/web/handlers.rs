use crate::movies::{
    data::store::MoviesStore,
    web::{
        errors::MoviesWebError,
        models::MovieViewModel,
        service::WebService,
        templates::{
            BaseTemplate, HomeTemplate, HtmxMovieDetailsTemplate, HtmxMoviesTemplate,
            MovieDetailsTemplate, MoviesTemplate,
        },
    },
};

use askama::Template;
use axum::{Extension, extract::Path, http::StatusCode, response::Html};
use tracing::{info, instrument};

#[instrument]
pub async fn movie_details(
    Extension(store): Extension<MoviesStore>,
    Path(movie_id): Path<i32>,
) -> Result<Html<String>, MoviesWebError> {
    let service = WebService::new(&store.connection);

    let movie = service
        .get_movie(movie_id)
        .await?
        .ok_or_else(|| MoviesWebError::NotFound(movie_id))?;

    let movie_details_template = MovieDetailsTemplate { movie }.render()?;

    info!("queried movie {}", movie_id);
    Ok(Html(movie_details_template))
}

#[instrument]
pub async fn list_movies(
    Extension(store): Extension<MoviesStore>,
) -> Result<Html<String>, MoviesWebError> {
    let service = WebService::new(&store.connection);

    let movies = service.get_movies().await?;

    let movies_template = MoviesTemplate { movies }.render()?;

    info!("queried all movies");
    Ok(Html(movies_template))
}

#[instrument]
pub async fn htmx_movie_details(
    Extension(store): Extension<MoviesStore>,
    Path(movie_id): Path<i32>,
) -> Result<Html<String>, MoviesWebError> {
    let service = WebService::new(&store.connection);

    let movie = service
        .get_movie(movie_id)
        .await?
        .ok_or_else(|| MoviesWebError::NotFound(movie_id))?;

    let htmx_movie_details_template = HtmxMovieDetailsTemplate { movie }.render()?;

    info!("htmx queried movie {}", movie_id);
    Ok(Html(htmx_movie_details_template))
}

#[instrument]
pub async fn htmx_list_movies(
    Extension(store): Extension<MoviesStore>,
) -> Result<Html<String>, MoviesWebError> {
    let service = WebService::new(&store.connection);

    let movies = service.get_movies().await?;

    let htmx_movies_template = HtmxMoviesTemplate { movies }.render()?;

    info!("htmx queried all movies");
    Ok(Html(htmx_movies_template))
}
