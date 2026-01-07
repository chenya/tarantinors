use crate::movies::web::{models::ErrorViewModel, templates::ErrorTemplate};
use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MoviesWebError {
    #[error("Template rendering failed: {0}")]
    RenderError(#[from] askama::Error),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Movie not found: {0}")]
    NotFound(i32),

    #[error("Invalid input: {0}")]
    ValidationError(String),

    #[error("Internal server error: {0}")]
    InternalError(String),
}

impl MoviesWebError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::ValidationError(_) => StatusCode::BAD_REQUEST,
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::RenderError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn user_message(&self) -> String {
        match self {
            Self::NotFound(id) => format!("Movie with ID {} not found", id),
            Self::ValidationError(msg) => msg.clone(),
            Self::DatabaseError(_) => "A database error occurred".to_string(),
            Self::RenderError(_) => "Failed to render page".to_string(),
            Self::InternalError(_) => "An internal error occurred".to_string(),
        }
    }

    pub fn log_message(&self) -> String {
        format!("{:?}", self)
    }
}

impl IntoResponse for MoviesWebError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let user_msg = self.user_message();
        let log_msg = self.log_message();

        // Log the error for debugging
        tracing::error!(
            error = %log_msg,
            status = %status,
            "Template error occurred"
        );

        let error_view_model = ErrorViewModel {
            code: status.as_u16(),
            message: user_msg.clone(),
            details: if cfg!(debug_assertions) {
                Some(log_msg)
            } else {
                None
            },
            show_suggestions: true,
            title: "Something Went Wrong".to_string(),
        };

        match (ErrorTemplate {
            error: error_view_model,
        })
        .render()
        {
            Ok(html) => (status, Html(html)).into_response(),
            Err(e) => {
                // Fallback if error template fails
                tracing::error!("Failed to render error template: {}", e);
                (
                    status,
                    Html(format!(
                        r#"
                        <!DOCTYPE html>
                        <html>
                        <head><title>Error</title></head>
                        <body>
                            <h1>Error {}</h1>
                            <p>{}</p>
                        </body>
                        </html>
                        "#,
                        status.as_u16(),
                        user_msg
                    )),
                )
                    .into_response()
            }
        }
    }
}
