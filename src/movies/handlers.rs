use super::models::{
    Actors, Directors, MovieAwards, MovieInput, MovieNominations, Producers, Writers,
};
use crate::{AppState, Msg};
use axum::Extension;
use axum::Json;
use axum::extract::Path;
use std::fmt::format;
use tracing::instrument;

use super::store::MoviesStore;

#[instrument]
pub async fn add_movie(
    Extension(store): Extension<MoviesStore>,
    Json(new_movie): Json<MovieInput>,
) -> Json<Msg> {
    let movie_title = new_movie.title.clone();
    match store.insert_movie(new_movie).await {
        Ok(_) => {
            let msg = format!("Movie '{}' added", movie_title);
            tracing::info!(msg);
            Json(Msg { message: msg })
        }
        Err(e) => {
            let err_msg = format!("Failed to add movie {}: {}", movie_title, e);
            tracing::error!(err_msg);
            Json(Msg { message: err_msg })
        }
    }
}

#[instrument]
pub async fn get_movie(
    Extension(store): Extension<MoviesStore>,
    Path(movie_id): Path<i32>,
) -> Json<MovieInput> {
    let movie = store.get_movie(movie_id).await.unwrap();

    tracing::info!(found = movie.is_some(), "queried movie");
    Json::from(movie.unwrap())
}

#[instrument]
pub async fn get_movies(Extension(store): Extension<MoviesStore>) -> Json<Vec<MovieInput>> {
    let movies = store.get_movies().await.unwrap();

    tracing::info!("queried all movies");
    Json::from(movies)
}

#[instrument]
pub async fn get_movie_actors(
    Extension(store): Extension<MoviesStore>,
    Path(movie_id): Path<i32>,
) -> Json<Actors> {
    let movie_actors = store.get_movie_actors(movie_id).await.unwrap();

    tracing::info!("queried movie actors");
    Json::from(Actors {
        actors: movie_actors,
    })
}

#[instrument]
pub async fn get_movie_directors(
    Extension(store): Extension<MoviesStore>,
    Path(movie_id): Path<i32>,
) -> Json<Directors> {
    let movie_directors = store.get_movie_directors(movie_id).await.unwrap();

    tracing::info!("queried movie directors");
    Json::from(Directors {
        directors: movie_directors,
    })
}

#[instrument]
pub async fn get_movie_producers(
    Extension(store): Extension<MoviesStore>,
    Path(movie_id): Path<i32>,
) -> Json<Producers> {
    let movie_producers = store.get_movie_producers(movie_id).await.unwrap();

    tracing::info!("queried movie producers");
    Json::from(Producers {
        producers: movie_producers,
    })
}

#[instrument]
pub async fn get_movie_writers(
    Extension(store): Extension<MoviesStore>,
    Path(movie_id): Path<i32>,
) -> Json<Writers> {
    let movie_writers = store.get_movie_writers(movie_id).await.unwrap();

    tracing::info!("queried movie writers");
    Json::from(Writers {
        writers: movie_writers,
    })
}

#[instrument]
pub async fn get_movie_awards(
    Extension(store): Extension<MoviesStore>,
    Path(movie_id): Path<i32>,
) -> Json<MovieAwards> {
    let movie_awards = store.get_movie_awards(movie_id).await.unwrap();

    tracing::info!("queried movie awards");
    Json::from(MovieAwards {
        awards: movie_awards,
    })
}

#[instrument]
pub async fn get_movie_nominations(
    Extension(store): Extension<MoviesStore>,
    Path(movie_id): Path<i32>,
) -> Json<MovieNominations> {
    let movie_nominations = store.get_movie_nominations(movie_id).await.unwrap();

    tracing::info!("queried movie nominations");
    Json::from(MovieNominations {
        nominations: movie_nominations,
    })
}

#[instrument]
pub async fn remove_movie(
    Extension(store): Extension<MoviesStore>,
    Path(movie_id): Path<i32>,
) -> Json<Msg> {
    let delete_result = store.delete_movie(movie_id).await;
    let msg = match delete_result {
        Ok(true) => format!("movie {} deleted", movie_id),
        Ok(false) => format!("movie {} not deleted", movie_id),
        Err(e) => format!("error deleting movie {}: {}", movie_id, e),
    };

    Json(Msg { message: msg })
}
