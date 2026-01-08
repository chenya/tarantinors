use crate::quotes::data::entities::Quote;
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::{PgPool, Postgres, Transaction};

pub struct QuoteRepository {
    pub pool: PgPool,
}

impl QuoteRepository {
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn create_quote(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        text: String,
    ) -> Result<Quote, sqlx::Error> {
        sqlx::query_as!(
            Quote,
            r#"
            INSERT INTO quote (text ) VALUES ($1)
            ON CONFLICT (text) DO UPDATE SET text = EXCLUDED.text
            RETURNING *
            "#,
            text
        )
        .fetch_one(&mut **tx)
        .await
    }

    pub async fn get_quote(&self, quote_id: i32) -> Result<Option<Quote>, sqlx::Error> {
        sqlx::query_as!(
            Quote,
            r#"
            SELECT * FROM quote WHERE id = $1
            "#,
            quote_id
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn get_quotes(&self) -> Result<Vec<Quote>, sqlx::Error> {
        sqlx::query_as!(
            Quote,
            r#"
            SELECT * FROM quote ORDER BY text ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn delete_quote(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        quote_id: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM quote WHERE id = $1
            "#,
            quote_id
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}
