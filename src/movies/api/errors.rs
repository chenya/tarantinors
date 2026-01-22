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
pub struct ApiErrorResponse {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<ValidationDetails>,
}

impl MoviesApiError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::MovieNotFound(_) => StatusCode::NOT_FOUND,
            Self::NoPersonsFoundForRole(_, _) => StatusCode::NOT_FOUND,
            Self::Validation(_) => StatusCode::BAD_REQUEST,
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::MovieNotFound(id) => format!("Movie with ID {} not found", id),
            Self::Validation(_) => "Request validation failed".to_string(),
            Self::NoPersonsFoundForRole(role, movie_id) => format!(
                "Requested persons of role '{}' for movie id({}) not found",
                role, movie_id
            ),
            Self::DatabaseError(e) => format!("Database error: {}", e),
            Self::InternalError(e) => format!("Internal error: {}", e),
        }
    }

    pub fn details(&self) -> Option<ValidationDetails> {
        match self {
            Self::MovieNotFound(_) => None,
            Self::Validation(errors) => {
                let field_errors = self.extract_field_errors(&errors);
                let error_count = field_errors.values().map(|v| v.len()).sum();

                Some(ValidationDetails {
                    field_errors,
                    error_count,
                })
            }
            Self::NoPersonsFoundForRole(_, _) => None,
            Self::DatabaseError(_) => None,
            Self::InternalError(_) => None,
        }
    }

    fn extract_field_errors(&self, errors: &ValidationErrors) -> HashMap<String, Vec<String>> {
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
}

impl IntoResponse for MoviesApiError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let message = self.message();
        let details = self.details();

        error!(message);
        let response = ApiErrorResponse { message, details };

        (status, Json(response)).into_response()
    }
}

