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

#[derive(Serialize, ToSchema)]
pub struct QuoteApiErrorResponse {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<ValidationDetails>,
}

#[derive(Serialize, ToSchema)]
pub struct ValidationDetails {
    pub field_errors: HashMap<String, Vec<String>>,
    pub error_count: usize,
}

#[derive(Debug, Error)]
pub enum QuotesApiError {
    #[error("quote not found")]
    NotFound(i32),

    #[error("Validation failed")]
    Validation(#[from] ValidationErrors),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Internal error: {0}")]
    InternalError(#[from] anyhow::Error),
}

impl QuotesApiError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Validation(_) => StatusCode::BAD_REQUEST,
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn user_message(&self) -> String {
        match self {
            Self::NotFound(id) => format!("Quote with ID {} not found", id),
            Self::Validation(_) => "Request validation failed".to_string(),
            Self::DatabaseError(_) => "A database error occurred".to_string(),
            Self::InternalError(_) => "An internal error occurred".to_string(),
        }
    }

    pub fn validation_details(&self) -> Option<ValidationDetails> {
        match self {
            Self::NotFound(id) => None,
            Self::DatabaseError(_) => None,
            Self::InternalError(_) => None,
            Self::Validation(errors) => {
                let field_errors = self.extract_field_errors(&errors);
                let error_count = field_errors.values().map(|v| v.len()).sum();

                Some(ValidationDetails {
                    field_errors,
                    error_count,
                })
            }
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


    pub fn log_message(&self) -> String {
        format!("{:?}", self)
    }
}

impl IntoResponse for QuotesApiError {
    fn into_response(self) -> Response {
        let status_code = self.status_code();
        let user_message = self.user_message();
        let log_message = self.log_message();
        let validation_details = self.validation_details();

        error!("{}", user_message);

        let response = QuoteApiErrorResponse {
            message: user_message,
            details: validation_details,
        };

        (status_code, Json(response)).into_response()
    }
}

