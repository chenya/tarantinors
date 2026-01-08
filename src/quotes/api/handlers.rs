// use crate::quotes::api::errors::{ApiErrorResponse, QuotesApiError};

use crate::quotes::api::models::{
    CreateQuoteRequest, QuoteListResponse, QuoteMessage, QuoteResponse,
};
use crate::quotes::api::service::ApiService;
use crate::quotes::data::store::QuoteStore;
use axum::Extension;
use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;

use tracing::{error, info, instrument};

#[instrument]
pub async fn add_new_quote(
    Extension(store): Extension<QuoteStore>,
    Json(new_quote): Json<CreateQuoteRequest>,
) -> impl IntoResponse {
    let service = ApiService::new(&store.connection);
    let quote_text = new_quote.text.clone();
    let _ = service.create_quote(new_quote).await.unwrap();

    let message = format!("Quote '{}' added ", quote_text);
    info!(%message);
    (StatusCode::CREATED, Json(QuoteMessage { message }))
}

#[instrument]
pub async fn get_quote(
    Extension(store): Extension<QuoteStore>,
    Path(quote_id): Path<i32>,
) -> Json<QuoteResponse> {
    // let movie = store.get_movie(movie_id).await?;

    let service = ApiService::new(&store.connection);

    let quote = service.get_quote(quote_id).await.unwrap().unwrap();
    // .ok_or(MoviesApiError::MovieNotFound(movie_id))?;

    info!("queried quote {quote_id}");
    Json(quote)
}

#[instrument]
pub async fn get_quotes(Extension(store): Extension<QuoteStore>) -> impl IntoResponse {
    // let movie = store.get_movie(movie_id).await?;

    let service = ApiService::new(&store.connection);

    let quotes = service.get_quotes().await.unwrap();
    // .ok_or(MoviesApiError::MovieNotFound(movie_id))?;

    info!("queried all quotes");
    (StatusCode::OK, Json(quotes))
}

#[instrument]
pub async fn remove_quote(
    Extension(store): Extension<QuoteStore>,
    Path(quote_id): Path<i32>,
) -> impl IntoResponse {
    // let movie = store.get_movie(movie_id).await?;

    let service = ApiService::new(&store.connection);

    let _ = service.delete_quote(quote_id).await.unwrap();
    // .ok_or(MoviesApiError::MovieNotFound(movie_id))?;

    let message = format!("Quote {quote_id} deleted");

    info!(%message);
    (StatusCode::OK, Json(QuoteMessage { message }))
}
