use chrono::{NaiveDate, NaiveDateTime};
use sqlx::PgPool;

#[derive(Clone, Debug)]
pub struct MoviesStore {
    pub connection: PgPool,
}

impl MoviesStore {
    pub fn new(connection: PgPool) -> Self {
        Self { connection }
    }
}
