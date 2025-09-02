use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Postgres, Transaction};
use crate::movies::models::{CreatePerson, Person};


pub async fn create_person(
    tx: &mut Transaction<'_, Postgres>,
    new_person: CreatePerson,
) -> Result<i32, sqlx::Error> {
    sqlx::query_scalar!(
            "INSERT INTO person (name ) VALUES ($1)
            ON CONFLICT (name) DO UPDATE SET name = EXCLUDED.name
            RETURNING id",
            new_person.name()
        )
        .fetch_one(&mut **tx)
        .await
}