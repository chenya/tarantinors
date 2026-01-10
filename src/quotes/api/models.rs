use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::{Validate, ValidationError};

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct QuoteResponse {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate, ToSchema)]
pub struct CreateQuoteRequest {
    #[validate(length(min = 1, message = "Text cannot be empty"))]
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct QuoteMessage {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct QuoteListResponse {
    pub quotes: Vec<QuoteResponse>,
}
