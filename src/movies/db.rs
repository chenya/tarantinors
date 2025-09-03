use crate::movies::models::{
    Award, CreateAward, CreateGenre, CreateMovie, CreatePerson, Genre, Movie, MovieGenre,
    MovieRole, Person, PersonRole,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Postgres, Transaction};

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

pub async fn create_genre(
    tx: &mut Transaction<'_, Postgres>,
    new_genre: CreateGenre,
) -> Result<i32, sqlx::Error> {
    sqlx::query_scalar!(
        "INSERT INTO genre (name ) VALUES ($1)
            ON CONFLICT (name) DO UPDATE SET name = EXCLUDED.name
            RETURNING id",
        new_genre.name()
    )
    .fetch_one(&mut **tx)
    .await
}

pub async fn create_award(
    tx: &mut Transaction<'_, Postgres>,
    new_award: CreateAward,
) -> Result<i32, sqlx::Error> {
    sqlx::query_scalar!(
        r#"
            INSERT INTO award (name) VALUES  ($1)
            ON CONFLICT (name) DO UPDATE SET name = EXCLUDED.name
            RETURNING id
            "#,
        new_award.name()
    )
    .fetch_one(&mut **tx)
    .await
}

async fn create_movie_role(
    tx: &mut Transaction<'_, Postgres>,
    movie_id: i32,
    person_id: i32,
    role: PersonRole,
) -> Result<MovieRole, sqlx::Error> {
    sqlx::query_as!(
        MovieRole,
        r#"
            INSERT INTO movie_role (movie_id, person_id, role_id)
            VALUES ($1, $2, (SELECT id FROM role WHERE name = $3))
            ON CONFLICT (movie_id, person_id, role_id)
            DO UPDATE SET role_id = EXCLUDED.role_id
            RETURNING movie_id, person_id, role_id
            "#,
        movie_id,
        person_id,
        role.to_string()
    )
    .fetch_one(&mut **tx)
    .await
}
pub async fn create_actor(
    tx: &mut Transaction<'_, Postgres>,
    movie_id: i32,
    person_id: i32,
) -> Result<MovieRole, sqlx::Error> {
    create_movie_role(tx, movie_id, person_id, PersonRole::Actor).await
}

pub async fn create_writer(
    tx: &mut Transaction<'_, Postgres>,
    movie_id: i32,
    person_id: i32,
) -> Result<MovieRole, sqlx::Error> {
    create_movie_role(tx, movie_id, person_id, PersonRole::Writer).await
}

pub async fn create_producer(
    tx: &mut Transaction<'_, Postgres>,
    movie_id: i32,
    person_id: i32,
) -> Result<MovieRole, sqlx::Error> {
    create_movie_role(tx, movie_id, person_id, PersonRole::Producer).await
}

pub async fn create_director(
    tx: &mut Transaction<'_, Postgres>,
    movie_id: i32,
    person_id: i32,
) -> Result<MovieRole, sqlx::Error> {
    create_movie_role(tx, movie_id, person_id, PersonRole::Director).await
}

pub async fn create_movie(
    tx: &mut Transaction<'_, Postgres>,
    new_movie: CreateMovie,
) -> Result<i32, sqlx::Error> {
    sqlx::query_scalar!(
        r#"
        INSERT INTO movie (
            title, release_year,  plot, runtime, rating,
            release_date, image_url, youtube_id, budget, production_details
        ) VALUES (
            $1,$2,$3,$4,$5,$6,$7,$8,$9,$10
        )
        ON CONFLICT (title) DO UPDATE SET title = EXCLUDED.title
        RETURNING id
        "#,
        new_movie.title(),
        new_movie.release_year(),
        new_movie.plot(),
        new_movie.runtime(),
        new_movie.rating(),
        new_movie.release_date(),
        new_movie.image_url(),
        new_movie.youtube_id(),
        new_movie.budget(),
        new_movie.production_details()
    )
    .fetch_one(&mut **tx)
    .await
}

pub async fn create_movie_genre(
    tx: &mut Transaction<'_, Postgres>,
    movie_id: i32,
    genre_id: i32,
) -> Result<MovieGenre, sqlx::Error> {
    sqlx::query_as!(
        MovieGenre,
        r#"
        INSERT INTO movie_genre (movie_id, genre_id)
        VALUES ($1,$2)
        RETURNING *
        "#,
        movie_id,
        genre_id
    )
    .fetch_one(&mut **tx)
    .await
}

pub async fn create_award_category(
    tx: &mut Transaction<'_, Postgres>,
    award_id: i32,
    category_name: &str,
) -> Result<i32, sqlx::Error> {
    sqlx::query_scalar!(
            r#"INSERT INTO award_category (award_id, category) VALUES ($1, $2)
            ON CONFLICT (award_id, category) DO UPDATE SET category = EXCLUDED.category
            RETURNING id"#,
            award_id,
            category_name
        )
        .fetch_one(&mut **tx)
        .await
}


pub async fn create_movie_award(
    tx: &mut Transaction<'_, Postgres>,
    movie_id: i32,
    award_category_id: i32,
    year: i32,
    recipient: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"INSERT INTO movie_award (movie_id, award_category_id, year, recipient)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT DO NOTHING
            "#,
        movie_id,
        award_category_id,
        year,
        recipient
    )
    .execute(&mut **tx)
    .await?;
    Ok(())
}

pub async fn create_movie_nomination(
    tx: &mut Transaction<'_, Postgres>,
    movie_id: i32,
    award_category_id: i32,
    year: i32,
    nominee: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"INSERT INTO movie_nomination (movie_id, award_category_id, year, nominee)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT DO NOTHING"#,
        movie_id,
        award_category_id,
        year,
        nominee
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}
