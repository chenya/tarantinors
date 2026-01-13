use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InterviewViewModel {
    pub title: String,
    pub description: String,
    pub youtube_id: String,
}
