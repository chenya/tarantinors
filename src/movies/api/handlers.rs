use crate::movies::api::errors::{ApiErrorResponse, MoviesApiError};
use crate::movies::api::extractors::ValidatedJson;
use crate::movies::api::models::{
    Actors, CreateMovieAwardNominationRequest, CreateMovieAwardRequest, CreateMovieRequest,
    Directors, MovieAwardNominationResponse, MovieAwardResponse, MovieAwardsResponse,
    MovieListResponse, MovieNominationsResponse, MovieResponse, MoviesMessage, Producers, Writers,
};
use crate::movies::api::service::ApiService;

use crate::store::Store;
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
    request_body = CreateMovieRequest,
    responses(
        (status = 201, description = "Movie Created", body = CreateMovieRequest),
        (status = 400, description = "Request Validation Error", body = ApiErrorResponse),
        (status = 500, description = "Internal server error", body = ApiErrorResponse)
    ),
    tag = "Movies API"
)]
#[instrument]
pub async fn add_movie(
    Extension(store): Extension<Store>,
    ValidatedJson(new_movie): ValidatedJson<CreateMovieRequest>,
) -> Result<impl IntoResponse, MoviesApiError> {
    let movie_title = new_movie.title.clone();
    let service = ApiService::new(&store.connection);
    let _ = service.create_movie(new_movie).await?;

    let message = format!("Movie '{}' added ", movie_title);
    info!(%message);
    Ok((StatusCode::CREATED, Json(MoviesMessage { message })))
}

/// Get movie by ID
#[utoipa::path(
    get,
    path = "/movies/{movie_id}",
    responses(
        (status = 200, description = "Movie found", body = MovieResponse),
        (status = 404, description = "Movie not found", body = ApiErrorResponse),
        (status = 500, description = "Database server error", body = ApiErrorResponse)
    ),
    tag = "Movies API"
)]
#[instrument]
pub async fn get_movie(
    Extension(store): Extension<Store>,
    Path(movie_id): Path<i32>,
) -> Result<Json<MovieResponse>, MoviesApiError> {
    // let movie = store.get_movie(movie_id).await?;

    let service = ApiService::new(&store.connection);

    let movie = service
        .get_movie(movie_id)
        .await?
        .ok_or_else(|| MoviesApiError::MovieNotFound(movie_id))?;

    info!("queried movie {movie_id}");
    Ok(Json(movie))
}

/// Get list of movies
#[utoipa::path(
    get,
    path = "/movies",
    responses(
        (status = 200, description = "List of Movies", body = MovieListResponse),
        (status = 500, description = "Database server error", body = ApiErrorResponse)
    ),
    tag = "Movies API"
)]
#[instrument]
pub async fn get_movies(
    Extension(store): Extension<Store>,
) -> Result<Json<MovieListResponse>, MoviesApiError> {
    // let movies = store.get_movies().await?;
    let service = ApiService::new(&store.connection);
    let movies = service.get_movies().await?;
    info!("queried all movies");
    Ok(Json::from(MovieListResponse { movies }))
}

/// Get movie actors by ID
#[utoipa::path(
    get,
    path = "/movies/{movie_id}/actors",
    responses(
        (status = 200, description = "List of movie actors", body = Actors),
        (status = 404, description = "Movie not found", body = ApiErrorResponse),
        (status = 500, description = "Database server error", body = ApiErrorResponse)
    ),
    tag = "Movies API"
)]
#[instrument]
pub async fn get_movie_actors(
    Extension(store): Extension<Store>,
    Path(movie_id): Path<i32>,
) -> Result<Json<Actors>, MoviesApiError> {
    let service = ApiService::new(&store.connection);

    let movie_actors = service.get_movie_actors(movie_id).await?;

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
        (status = 404, description = "Movie not found", body = ApiErrorResponse),
        (status = 500, description = "Database server error", body = ApiErrorResponse)
    ),
    tag = "Movies API"
)]
#[instrument]
pub async fn get_movie_directors(
    Extension(store): Extension<Store>,
    Path(movie_id): Path<i32>,
) -> Result<Json<Directors>, MoviesApiError> {
    let service = ApiService::new(&store.connection);
    let movie_directors = service.get_movie_directors(movie_id).await?;

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
        (status = 404, description = "Movie not found", body = ApiErrorResponse),
        (status = 500, description = "Database server error", body = ApiErrorResponse)
    ),
    tag = "Movies API"
)]
#[instrument]
pub async fn get_movie_producers(
    Extension(store): Extension<Store>,
    Path(movie_id): Path<i32>,
) -> Result<Json<Producers>, MoviesApiError> {
    let service = ApiService::new(&store.connection);
    let movie_producers = service.get_movie_producers(movie_id).await?;

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
        (status = 404, description = "Movie not found", body = ApiErrorResponse),
        (status = 500, description = "Database server error", body = ApiErrorResponse)
    ),
    tag = "Movies API"
)]
#[instrument]
pub async fn get_movie_writers(
    Extension(store): Extension<Store>,
    Path(movie_id): Path<i32>,
) -> Result<Json<Writers>, MoviesApiError> {
    let service = ApiService::new(&store.connection);
    let movie_writers = service.get_movie_writers(movie_id).await?;

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
        (status = 200, description = "List of movie winning awards", body = MovieAwardsResponse),
        (status = 404, description = "Movie not found", body = ApiErrorResponse),
        (status = 500, description = "Database server error", body = ApiErrorResponse)
    ),
    tag = "Movies API"
)]
#[instrument]
pub async fn get_movie_awards(
    Extension(store): Extension<Store>,
    Path(movie_id): Path<i32>,
) -> Result<Json<MovieAwardsResponse>, MoviesApiError> {
    let service = ApiService::new(&store.connection);

    let movie_awards = service.get_movie_awards(movie_id).await?;

    // if movie_awards.is_empty() && !store.is_movie_id_exists(movie_id).await? {
    //     return Err(MoviesApiError::MovieNotFound(movie_id));
    // }

    info!("queried movie {movie_id} awards");
    Ok(Json::from(MovieAwardsResponse {
        awards: movie_awards,
    }))
}

/// Get movie nominations awards by ID
#[utoipa::path(
    get,
    path = "/movies/{movie_id}/nominations",
    responses(
        (status = 200, description = "List of movie nominations awards", body = MovieNominationsResponse),
        (status = 404, description = "Movie not found", body = ApiErrorResponse),
        (status = 500, description = "Database server error", body = ApiErrorResponse)
    ),
    tag = "Movies API"
)]
#[instrument]
pub async fn get_movie_nominations(
    Extension(store): Extension<Store>,
    Path(movie_id): Path<i32>,
) -> Result<Json<MovieNominationsResponse>, MoviesApiError> {
    let service = ApiService::new(&store.connection);
    let movie_nominations = service.get_movie_nominations(movie_id).await?;

    // if movie_nominations.is_empty() && !store.is_movie_id_exists(movie_id).await? {
    //     return Err(MoviesApiError::MovieNotFound(movie_id));
    // }

    info!("queried movie {movie_id} nominations");
    Ok(Json::from(MovieNominationsResponse {
        nominations: movie_nominations,
    }))
}

/// Delete movie by ID
#[utoipa::path(
    delete,
    path = "/movies/{movie_id}",
    responses(
        (status = 200, description = "Movie Deleted", body = MoviesMessage),
        (status = 404, description = "Movie not found", body = ApiErrorResponse),
        (status = 500, description = "Database server error", body = ApiErrorResponse)
    ),
    tag = "Movies API"
)]
#[instrument]
pub async fn remove_movie(
    Extension(store): Extension<Store>,
    Path(movie_id): Path<i32>,
) -> Result<Json<MoviesMessage>, MoviesApiError> {
    let service = ApiService::new(&store.connection);
    let _ = service.delete_movie(movie_id).await?;

    let message = format!("Movie {movie_id} deleted");

    info!(%message);
    Ok(Json(MoviesMessage { message }))
}

pub async fn fallback_handler(uri: axum::http::Uri) -> impl IntoResponse {
    let message = format!("The requested resource: '{}' could not be found", uri);

    let error_response = ApiErrorResponse {
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
    components(schemas(
        CreateMovieRequest,
        CreateMovieAwardRequest,
        CreateMovieAwardNominationRequest,
        MovieAwardResponse,
        MovieAwardNominationResponse,
        CreateMovieRequest,
        MovieResponse,
        Actors,
        Directors,
        Producers,
        Writers,
        MovieAwardsResponse,
        MovieNominationsResponse,
        MoviesMessage,
        MovieListResponse,
    )),
    modifiers()
)]
pub struct MoviesApiDoc;
