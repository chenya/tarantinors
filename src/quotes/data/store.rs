use chrono::{NaiveDate, NaiveDateTime};
use sqlx::PgPool;

#[derive(Clone, Debug)]
pub struct QuoteStore {
    pub connection: PgPool,
}

impl QuoteStore {
    pub fn new(connection: PgPool) -> Self {
        Self { connection }
    }
}
