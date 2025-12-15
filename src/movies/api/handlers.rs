use crate::movies::api::errors::{ErrorResponse, MoviesApiError};
use crate::movies::api::extractors::ValidatedJson;
use crate::movies::api::models::{
    Actors, Directors, MovieAwardNomination, MovieAwardWon, MovieAwards, MovieInput,
    MovieNominations, Movies, MoviesMessage, Producers, Writers,
};
use crate::movies::data::store::MoviesStore;

use axum::Extension;
use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;

use tracing::{error, info, instrument};
use utoipa::OpenApi;

/// Create a new movie
#[utoipa::path(
    post,
    path = "/movies",
    request_body = MovieInput,
    responses(
        (status = 201, description = "Movie Created", body = MovieInput),
        (status = 400, description = "Request Validation Error", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Movies API"
)]
#[instrument]
pub async fn add_movie(
    Extension(store): Extension<MoviesStore>,
    ValidatedJson(new_movie): ValidatedJson<MovieInput>,
) -> Result<impl IntoResponse, MoviesApiError> {
    let movie_title = new_movie.title.clone();
    store.insert_movie(new_movie).await?;

    let message = format!("Movie '{}' added ", movie_title);
    info!(%message);
    Ok((StatusCode::CREATED, Json(MoviesMessage { message })))
}

/// Get movie by ID
#[utoipa::path(
    get,
    path = "/movies/{movie_id}",
    responses(
        (status = 200, description = "Movie found", body = MovieInput),
        (status = 404, description = "Movie not found", body = ErrorResponse),
        (status = 500, description = "Database server error", body = ErrorResponse)
    ),
    tag = "Movies API"
)]
#[instrument]
pub async fn get_movie(
    Extension(store): Extension<MoviesStore>,
    Path(movie_id): Path<i32>,
) -> Result<Json<MovieInput>, MoviesApiError> {
    let movie = store.get_movie(movie_id).await?;

    info!("queried movie {movie_id}");
    Ok(Json(movie))
}

/// Get list of movies
#[utoipa::path(
    get,
    path = "/movies",
    responses(
        (status = 200, description = "List of Movies", body = Movies),
        (status = 500, description = "Database server error", body = ErrorResponse)
    ),
    tag = "Movies API"
)]
#[instrument]
pub async fn get_movies(
    Extension(store): Extension<MoviesStore>,
) -> Result<Json<Movies>, MoviesApiError> {
    let movies = store.get_movies().await?;

    info!("queried all movies");
    Ok(Json::from(Movies { movies }))
}

/// Get movie actors by ID
#[utoipa::path(
    get,
    path = "/movies/{movie_id}/actors",
    responses(
        (status = 200, description = "List of movie actors", body = Actors),
        (status = 404, description = "Movie not found", body = ErrorResponse),
        (status = 500, description = "Database server error", body = ErrorResponse)
    ),
    tag = "Movies API"
)]
#[instrument]
pub async fn get_movie_actors(
    Extension(store): Extension<MoviesStore>,
    Path(movie_id): Path<i32>,
) -> Result<Json<Actors>, MoviesApiError> {
    let movie_actors = store.get_movie_actors(movie_id).await?;

    info!("queried movie {movie_id} actors");
    Ok(Json::from(Actors {
        actors: movie_actors,
    }))
}

/// Get movie directos by ID
#[utoipa::path(
    get,
    path = "/movies/{movie_id}/directors",
    responses(
        (status = 200, description = "List of movie directors", body = Directors),
        (status = 404, description = "Movie not found", body = ErrorResponse),
        (status = 500, description = "Database server error", body = ErrorResponse)
    ),
    tag = "Movies API"
)]
#[instrument]
pub async fn get_movie_directors(
    Extension(store): Extension<MoviesStore>,
    Path(movie_id): Path<i32>,
) -> Result<Json<Directors>, MoviesApiError> {
    let movie_directors = store.get_movie_directors(movie_id).await?;

    info!("queried movie {movie_id} directors");
    Ok(Json::from(Directors {
        directors: movie_directors,
    }))
}

/// Get movie producers by ID
#[utoipa::path(
    get,
    path = "/movies/{movie_id}/producers",
    responses(
        (status = 200, description = "List of movie producers", body = Producers),
        (status = 404, description = "Movie not found", body = ErrorResponse),
        (status = 500, description = "Database server error", body = ErrorResponse)
    ),
    tag = "Movies API"
)]
#[instrument]
pub async fn get_movie_producers(
    Extension(store): Extension<MoviesStore>,
    Path(movie_id): Path<i32>,
) -> Result<Json<Producers>, MoviesApiError> {
    let movie_producers = store.get_movie_producers(movie_id).await?;

    info!("queried movie {movie_id} producers");
    Ok(Json::from(Producers {
        producers: movie_producers,
    }))
}

/// Get movie writers by ID
#[utoipa::path(
    get,
    path = "/movies/{movie_id}/writers",
    responses(
        (status = 200, description = "List of movie writers", body = Writers),
        (status = 404, description = "Movie not found", body = ErrorResponse),
        (status = 500, description = "Database server error", body = ErrorResponse)
    ),
    tag = "Movies API"
)]
#[instrument]
pub async fn get_movie_writers(
    Extension(store): Extension<MoviesStore>,
    Path(movie_id): Path<i32>,
) -> Result<Json<Writers>, MoviesApiError> {
    let movie_writers = store.get_movie_writers(movie_id).await?;

    info!("queried movie {movie_id} writers");
    Ok(Json::from(Writers {
        writers: movie_writers,
    }))
}

/// Get movie winning awards by ID
#[utoipa::path(
    get,
    path = "/movies/{movie_id}/awards",
    responses(
        (status = 200, description = "List of movie winning awards", body = MovieAwards),
        (status = 404, description = "Movie not found", body = ErrorResponse),
        (status = 500, description = "Database server error", body = ErrorResponse)
    ),
    tag = "Movies API"
)]
#[instrument]
pub async fn get_movie_awards(
    Extension(store): Extension<MoviesStore>,
    Path(movie_id): Path<i32>,
) -> Result<Json<MovieAwards>, MoviesApiError> {
    let movie_awards = store.get_movie_awards(movie_id).await?;

    if movie_awards.is_empty() && !store.is_movie_id_exists(movie_id).await? {
        return Err(MoviesApiError::MovieNotFound(movie_id));
    }

    info!("queried movie {movie_id} awards");
    Ok(Json::from(MovieAwards {
        awards: movie_awards,
    }))
}

/// Get movie nominations awards by ID
#[utoipa::path(
    get,
    path = "/movies/{movie_id}/nominations",
    responses(
        (status = 200, description = "List of movie nominations awards", body = MovieNominations),
        (status = 404, description = "Movie not found", body = ErrorResponse),
        (status = 500, description = "Database server error", body = ErrorResponse)
    ),
    tag = "Movies API"
)]
#[instrument]
pub async fn get_movie_nominations(
    Extension(store): Extension<MoviesStore>,
    Path(movie_id): Path<i32>,
) -> Result<Json<MovieNominations>, MoviesApiError> {
    let movie_nominations = store.get_movie_nominations(movie_id).await?;

    if movie_nominations.is_empty() && !store.is_movie_id_exists(movie_id).await? {
        return Err(MoviesApiError::MovieNotFound(movie_id));
    }

    info!("queried movie {movie_id} nominations");
    Ok(Json::from(MovieNominations {
        nominations: movie_nominations,
    }))
}

/// Delete movie by ID
#[utoipa::path(
    delete,
    path = "/movies/{movie_id}",
    responses(
        (status = 200, description = "Movie Deleted", body = MoviesMessage),
        (status = 404, description = "Movie not found", body = ErrorResponse),
        (status = 500, description = "Database server error", body = ErrorResponse)
    ),
    tag = "Movies API"
)]
#[instrument]
pub async fn remove_movie(
    Extension(store): Extension<MoviesStore>,
    Path(movie_id): Path<i32>,
) -> Result<Json<MoviesMessage>, MoviesApiError> {
    let _ = store.delete_movie(movie_id).await?;

    let message = format!("Movie {movie_id} deleted");

    info!(%message);
    Ok(Json(MoviesMessage { message }))
}

pub async fn fallback_handler(uri: axum::http::Uri) -> impl IntoResponse {
    let message = format!("The requested resource: '{}' could not be found", uri);

    let error_response = ErrorResponse {
        message,
        details: None,
    };

    (StatusCode::NOT_FOUND, Json(error_response))
}

#[derive(OpenApi)]
#[openapi(
    paths(
        get_movies,
        get_movie,
        get_movie_actors,
        get_movie_directors,
        get_movie_producers,
        get_movie_writers,
        get_movie_awards,
        get_movie_nominations,
        add_movie,
        remove_movie
    ),
    components(
        schemas(
            MovieInput,
            MovieAwardWon,
            MovieAwardNomination,
            ErrorResponse,
            MoviesMessage,
            Movies,
            Actors,
            Producers,
            Directors,
            Writers,
            MovieAwards,
            MovieNominations,

        )
    ),
    modifiers(),
    tags(
        (name = "Movies", description = "Movies endpoints")
    )
)]
pub struct MoviesApiDoc;
