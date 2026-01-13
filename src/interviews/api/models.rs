use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::{Validate, ValidationError};

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct InterviewResponse {
    pub title: String,
    pub description: String,
    pub youtube_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate, ToSchema)]
pub struct CreateInterviewRequest {
    #[validate(length(min = 1, message = "Title cannot be empty"))]
    pub title: String,

    #[validate(length(min = 1, message = "Description cannot be empty"))]
    pub description: String,

    #[validate(length(min = 1, message = "YouTube ID cannot be empty"))]
    pub youtube_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct InterviewMessage {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct InterviewListResponse {
    pub interviews: Vec<InterviewResponse>,
}
