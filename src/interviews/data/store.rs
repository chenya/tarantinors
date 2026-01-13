use chrono::{NaiveDate, NaiveDateTime};
use sqlx::PgPool;

#[derive(Clone, Debug)]
pub struct InterviewStore {
    pub connection: PgPool,
}

impl InterviewStore {
    pub fn new(connection: PgPool) -> Self {
        Self { connection }
    }
}
