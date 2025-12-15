use crate::movies::api::models::MoviesMessage;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use thiserror::Error;
use tracing::error;
use utoipa::ToSchema;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum MoviesApiError {
    #[error("movie not found")]
    MovieNotFound(i32),

    #[error("Persons not found")]
    NoPersonsFoundForRole(String, i32),

    #[error("Validation failed")]
    Validation(#[from] ValidationErrors),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Internal error: {0}")]
    InternalError(#[from] anyhow::Error),
}

#[derive(Serialize, ToSchema)]
pub struct ValidationDetails {
    pub field_errors: HashMap<String, Vec<String>>,
    pub error_count: usize,
}

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<ValidationDetails>,
}

impl IntoResponse for MoviesApiError {
    fn into_response(self) -> Response {
        let (status, message, details) = match self {
            MoviesApiError::MovieNotFound(movie_id) => {
                error!("Requested movie id({}) not found", movie_id);
                (
                    StatusCode::NOT_FOUND,
                    format!("Requested movie id({}) not found", movie_id),
                    None,
                )
            }
            MoviesApiError::NoPersonsFoundForRole(role, movie_id) => {
                error!(
                    "Requested persons of role '{}' for movie id({}) not found",
                    role, movie_id
                );
                (
                    StatusCode::NOT_FOUND,
                    format!(
                        "Requested persons of role '{}' for movie id({}) not found",
                        role, movie_id
                    ),
                    None,
                )
            }
            MoviesApiError::Validation(errors) => {
                let field_errors = extract_field_errors(&errors);
                let error_count = field_errors.values().map(|v| v.len()).sum();
                error!("Request validation errors: {:?}", field_errors);
                (
                    StatusCode::BAD_REQUEST,
                    "Request validation failed".to_string(),
                    Some(ValidationDetails {
                        field_errors,
                        error_count,
                    }),
                )
            }

            MoviesApiError::DatabaseError(e) => {
                error!("Database error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string(), None)
            }

            MoviesApiError::InternalError(e) => {
                error!("Internal error: {e}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Internal error: {e}"),
                    None,
                )
            }
        };

        let response = ErrorResponse { message, details };

        (status, Json(response)).into_response()
    }
}

fn extract_field_errors(errors: &ValidationErrors) -> HashMap<String, Vec<String>> {
    errors
        .field_errors()
        .iter()
        .map(|(field, field_errors)| {
            let messages = field_errors
                .iter()
                .map(|error| {
                    error
                        .message
                        .as_ref()
                        .map(|cow| cow.to_string())
                        .unwrap_or_else(|| "Invalid value".to_string())
                })
                .collect();
            (field.to_string(), messages)
        })
        .collect()
}

#[derive(Debug, Error)]
pub enum MoviesError {
    #[error("movie not found")]
    NotFound,

    #[error("Failed database query: {0}")]
    DbQuery(#[from] sqlx::Error),

    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for MoviesError {
    fn into_response(self) -> Response {
        let (status_code, message) = match &self {
            MoviesError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),

            MoviesError::DbQuery(e) => {
                error!("database error: {:?}", self);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("database query error: {}", e),
                )
            }
            MoviesError::Internal(e) => {
                error!("internal error: {:?}", self);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("internal error {}", e),
                )
            }
        };

        let body = Json(MoviesMessage { message });
        (status_code, body).into_response()
    }
}
