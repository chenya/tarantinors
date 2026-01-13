use crate::interviews::data::entities::Interview;
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::{PgPool, Postgres, Transaction};

pub struct InterviewRepository {
    pub pool: PgPool,
}

impl InterviewRepository {
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn create_interview(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        title: String,
        description: String,
        youtube_id: String,
    ) -> Result<Interview, sqlx::Error> {
        sqlx::query_as!(
            Interview,
            r#"
            INSERT INTO interview (title, description, youtube_id) VALUES ($1, $2, $3)
            ON CONFLICT (title) DO UPDATE SET title = EXCLUDED.title
            RETURNING *
            "#,
            title,
            description,
            youtube_id
        )
        .fetch_one(&mut **tx)
        .await
    }

    pub async fn get_interview(&self, interview_id: i32) -> Result<Option<Interview>, sqlx::Error> {
        sqlx::query_as!(
            Interview,
            r#"
            SELECT * FROM interview WHERE id = $1
            "#,
            interview_id
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn get_interviews(&self) -> Result<Vec<Interview>, sqlx::Error> {
        sqlx::query_as!(
            Interview,
            r#"
            SELECT * FROM interview ORDER BY title ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn delete_interview(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        interview_id: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM interview WHERE id = $1
            "#,
            interview_id
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}
