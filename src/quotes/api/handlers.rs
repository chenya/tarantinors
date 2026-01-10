// use crate::quotes::api::errors::{ApiErrorResponse, QuotesApiError};

use crate::quotes::api::errors::{QuoteApiErrorResponse, QuotesApiError};
use crate::quotes::api::extractors::ValidatedJson;
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
use utoipa::OpenApi;

/// Create a new quote
#[utoipa::path(
    post,
    path = "/quotes",
    request_body = CreateQuoteRequest,
    responses(
        (status = 201, description = "Movie Created", body = CreateQuoteRequest),
        (status = 400, description = "Request Validation Error", body = QuoteApiErrorResponse),
        (status = 500, description = "Internal server error", body = QuoteApiErrorResponse)
    ),
    tag = "Quotes API"
)]
#[instrument]
pub async fn add_new_quote(
    Extension(store): Extension<QuoteStore>,
    ValidatedJson(new_quote): ValidatedJson<CreateQuoteRequest>,
) -> impl IntoResponse {
    let service = ApiService::new(&store.connection);
    let quote_text = new_quote.text.clone();
    let _ = service.create_quote(new_quote).await.unwrap();

    let message = format!("Quote '{}' added ", quote_text);
    info!(%message);
    (StatusCode::CREATED, Json(QuoteMessage { message }))
}

/// Get quote by ID
#[utoipa::path(
    get,
    path = "/quotes/{quote_id}",
    responses(
        (status = 200, description = "Quote found", body = QuoteResponse),
        (status = 404, description = "Quote not found", body = QuoteApiErrorResponse),
        (status = 500, description = "Database server error", body = QuoteApiErrorResponse)
    ),
    tag = "Quotes API"
)]
#[instrument]
pub async fn get_quote(
    Extension(store): Extension<QuoteStore>,
    Path(quote_id): Path<i32>,
) -> Result<impl IntoResponse, QuotesApiError> {
    // let movie = store.get_movie(movie_id).await?;

    let service = ApiService::new(&store.connection);

    let quote = service
        .get_quote(quote_id)
        .await?
        .ok_or(QuotesApiError::NotFound(quote_id))?;

    info!("queried quote {quote_id}");
    Ok(Json(quote))
}

/// Get list of quotes
#[utoipa::path(
    get,
    path = "/quotes",
    responses(
        (status = 200, description = "List of Quotes", body = QuoteListResponse),
        (status = 500, description = "Database server error", body = QuoteApiErrorResponse)
    ),
    tag = "Quotes API"
)]
#[instrument]
pub async fn get_quotes(
    Extension(store): Extension<QuoteStore>,
) -> Result<impl IntoResponse, QuotesApiError> {
    let service = ApiService::new(&store.connection);

    let quotes = service.get_quotes().await?;

    info!("queried all quotes");
    Ok(Json(quotes))
}

/// Delete quote by ID
#[utoipa::path(
    delete,
    path = "/quotes/{quote_id}",
    responses(
        (status = 200, description = "Quote Deleted", body = QuoteMessage),
        (status = 404, description = "Quote not found", body = QuoteApiErrorResponse),
        (status = 500, description = "Database server error", body = QuoteApiErrorResponse)
    ),
    tag = "Quotes API"
)]
#[instrument]
pub async fn remove_quote(
    Extension(store): Extension<QuoteStore>,
    Path(quote_id): Path<i32>,
) -> Result<impl IntoResponse, QuotesApiError> {
    let service = ApiService::new(&store.connection);

    let _ = service.delete_quote(quote_id).await?;

    let message = format!("Quote {quote_id} deleted");

    info!(%message);
    Ok(Json(QuoteMessage { message }))
}

#[derive(OpenApi)]
#[openapi(
    paths(add_new_quote, get_quote, get_quotes, remove_quote,),
    components(schemas()),
    modifiers()
)]
pub struct QuotesApiDoc;
