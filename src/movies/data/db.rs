// use crate::movies::data::models::{
//     CreateAward, CreateGenre, CreateMovie, CreatePerson, Genre, Movie, MovieAwardNomination,
//     MovieAwardWon, MovieGenre, MovieRole, Person, PersonRole,
// };

use crate::movies::api::models::{
    CreateAward, CreateGenre, CreateMovie, CreatePerson, Genre, Movie, MovieAwardNomination,
    MovieAwardWon, MovieGenre, MovieRole, Person, PersonRole,
};
use sqlx::{PgPool, Postgres, Transaction};

use crate::movies::api::errors::MoviesApiError;

pub async fn create_person(
    tx: &mut Transaction<'_, Postgres>,
    new_person: CreatePerson,
) -> Result<i32, MoviesApiError> {
    sqlx::query_scalar!(
        r#"
        INSERT INTO person (name ) VALUES ($1)
        ON CONFLICT (name) DO UPDATE SET name = EXCLUDED.name
        RETURNING id
        "#,
        new_person.name()
    )
    .fetch_one(&mut **tx)
    .await
    .map_err(|e| MoviesApiError::DatabaseError(e))
}

pub async fn create_genre(
    tx: &mut Transaction<'_, Postgres>,
    new_genre: CreateGenre,
) -> Result<i32, MoviesApiError> {
    sqlx::query_scalar!(
        r#"
        INSERT INTO genre (name ) VALUES ($1)
        ON CONFLICT (name) DO UPDATE SET name = EXCLUDED.name
        RETURNING id
        "#,
        new_genre.name()
    )
    .fetch_one(&mut **tx)
    .await
    .map_err(|e| MoviesApiError::DatabaseError(e))
}

pub async fn create_award(
    tx: &mut Transaction<'_, Postgres>,
    new_award: CreateAward,
) -> Result<i32, MoviesApiError> {
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
    .map_err(|e| MoviesApiError::DatabaseError(e))
}

async fn create_movie_role(
    tx: &mut Transaction<'_, Postgres>,
    movie_id: i32,
    person_id: i32,
    role: PersonRole,
) -> Result<MovieRole, MoviesApiError> {
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
    .map_err(|e| MoviesApiError::DatabaseError(e))
}
pub async fn create_actor(
    tx: &mut Transaction<'_, Postgres>,
    movie_id: i32,
    person_id: i32,
) -> Result<MovieRole, MoviesApiError> {
    create_movie_role(tx, movie_id, person_id, PersonRole::Actor).await
}

pub async fn create_writer(
    tx: &mut Transaction<'_, Postgres>,
    movie_id: i32,
    person_id: i32,
) -> Result<MovieRole, MoviesApiError> {
    create_movie_role(tx, movie_id, person_id, PersonRole::Writer).await
}

pub async fn create_producer(
    tx: &mut Transaction<'_, Postgres>,
    movie_id: i32,
    person_id: i32,
) -> Result<MovieRole, MoviesApiError> {
    create_movie_role(tx, movie_id, person_id, PersonRole::Producer).await
}

pub async fn create_director(
    tx: &mut Transaction<'_, Postgres>,
    movie_id: i32,
    person_id: i32,
) -> Result<MovieRole, MoviesApiError> {
    create_movie_role(tx, movie_id, person_id, PersonRole::Director).await
}

pub async fn create_movie(
    tx: &mut Transaction<'_, Postgres>,
    new_movie: CreateMovie,
) -> Result<i32, MoviesApiError> {
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
    .map_err(|e| MoviesApiError::DatabaseError(e))
}

pub async fn create_movie_genre(
    tx: &mut Transaction<'_, Postgres>,
    movie_id: i32,
    genre_id: i32,
) -> Result<MovieGenre, MoviesApiError> {
    sqlx::query_as!(
        MovieGenre,
        r#"
        INSERT INTO movie_genre (movie_id, genre_id)
        VALUES ($1,$2)
        ON CONFLICT (movie_id, genre_id) DO UPDATE SET genre_id = EXCLUDED.genre_id
        RETURNING *
        "#,
        movie_id,
        genre_id
    )
    .fetch_one(&mut **tx)
    .await
    .map_err(|e| MoviesApiError::DatabaseError(e))
}

pub async fn create_award_category(
    tx: &mut Transaction<'_, Postgres>,
    award_id: i32,
    category_name: &str,
) -> Result<i32, MoviesApiError> {
    sqlx::query_scalar!(
        r#"
        INSERT INTO award_category (award_id, category) VALUES ($1, $2)
        ON CONFLICT (award_id, category) DO UPDATE SET category = EXCLUDED.category
        RETURNING id
        "#,
        award_id,
        category_name
    )
    .fetch_one(&mut **tx)
    .await
    .map_err(|e| MoviesApiError::DatabaseError(e))
}

pub async fn create_movie_award(
    tx: &mut Transaction<'_, Postgres>,
    movie_id: i32,
    award_category_id: i32,
    year: i32,
    recipient: &str,
) -> Result<(), MoviesApiError> {
    sqlx::query!(
        r#"
        INSERT INTO movie_award (movie_id, award_category_id, year, recipient)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT DO NOTHING
        "#,
        movie_id,
        award_category_id,
        year,
        recipient
    )
    .execute(&mut **tx)
    .await
    .map_err(|e| MoviesApiError::DatabaseError(e))?;
    Ok(())
}

pub async fn create_movie_nomination(
    tx: &mut Transaction<'_, Postgres>,
    movie_id: i32,
    award_category_id: i32,
    year: i32,
    nominee: &str,
) -> Result<(), MoviesApiError> {
    sqlx::query!(
        r#"
        INSERT INTO movie_nomination (movie_id, award_category_id, year, nominee)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT DO NOTHING
        "#,
        movie_id,
        award_category_id,
        year,
        nominee
    )
    .execute(&mut **tx)
    .await
    .map_err(|e| MoviesApiError::DatabaseError(e))?;
    Ok(())
}

pub async fn is_movie_id_exists(pool: &PgPool, movie_id: i32) -> Result<bool, MoviesApiError> {
    let id_exists = sqlx::query_scalar!(
        r#"
        SELECT 1 FROM movie WHERE id = $1
        "#,
        movie_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| MoviesApiError::DatabaseError(e))?;

    Ok(id_exists.is_some())
}

pub async fn get_movie_by_id(pool: &PgPool, movie_id: i32) -> Result<Movie, MoviesApiError> {
    sqlx::query_as!(
        Movie,
        r#"
        SELECT * FROM movie WHERE id = $1
        "#,
        movie_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| MoviesApiError::DatabaseError(e))?
    .ok_or(MoviesApiError::MovieNotFound(movie_id))
}

async fn get_persons_by_role(
    pool: &PgPool,
    movie_id: i32,
    role: PersonRole,
) -> Result<Vec<Person>, MoviesApiError> {
    let _ = get_movie_by_id(pool, movie_id).await?;

    let persons = sqlx::query_as!(
        Person,
        r#"
        SELECT p.*
        FROM person p INNER JOIN movie_role mr ON p.id = mr.person_id
        WHERE mr.movie_id = $1 AND mr.role_id in (SELECT id FROM role WHERE name = $2)
        "#,
        movie_id,
        role.to_string()
    )
    .fetch_all(pool)
    .await
    .map_err(|e| MoviesApiError::DatabaseError(e))?;

    match persons.is_empty() {
        true => Err(MoviesApiError::NoPersonsFoundForRole(
            role.to_string(),
            movie_id,
        )),
        false => Ok(persons),
    }
}

async fn get_persons_names_by_role(
    pool: &PgPool,
    movie_id: i32,
    role: PersonRole,
) -> Result<Vec<String>, MoviesApiError> {
    let persons_names = get_persons_by_role(pool, movie_id, role)
        .await?
        .into_iter()
        .map(|p| p.name)
        .collect();
    Ok(persons_names)
}

pub async fn get_movie_actors_names(
    pool: &PgPool,
    movie_id: i32,
) -> Result<Vec<String>, MoviesApiError> {
    get_persons_names_by_role(pool, movie_id, PersonRole::Actor).await
}
pub async fn get_movie_directors_names(
    pool: &PgPool,
    movie_id: i32,
) -> Result<Vec<String>, MoviesApiError> {
    get_persons_names_by_role(pool, movie_id, PersonRole::Director).await
}
pub async fn get_movie_producers_names(
    pool: &PgPool,
    movie_id: i32,
) -> Result<Vec<String>, MoviesApiError> {
    get_persons_names_by_role(pool, movie_id, PersonRole::Producer).await
}
pub async fn get_movie_writers_names(
    pool: &PgPool,
    movie_id: i32,
) -> Result<Vec<String>, MoviesApiError> {
    get_persons_names_by_role(pool, movie_id, PersonRole::Writer).await
}

pub async fn get_movie_awards_won(
    pool: &PgPool,
    movie_id: i32,
) -> Result<Vec<MovieAwardWon>, MoviesApiError> {
    sqlx::query_as!(
        MovieAwardWon,
        r#"
        SELECT a.name AS name, ac.category, ma.year, ma.recipient
        FROM movie_award ma
                 JOIN award_category ac ON ma.award_category_id = ac.id
                 JOIN award a ON ac.award_id = a.id
        WHERE ma.movie_id = $1
        "#,
        movie_id
    )
    .fetch_all(pool)
    .await
    .map_err(|e| MoviesApiError::DatabaseError(e))
}

pub async fn get_movie_awards_nominations(
    pool: &PgPool,
    movie_id: i32,
) -> Result<Vec<MovieAwardNomination>, MoviesApiError> {
    sqlx::query_as!(
        MovieAwardNomination,
        r#"
        SELECT a.name AS name, ac.category, mn.year, mn.nominee
        FROM movie_nomination mn
                 JOIN award_category ac ON mn.award_category_id = ac.id
                 JOIN award a ON ac.award_id = a.id
        WHERE mn.movie_id = $1
        "#,
        movie_id
    )
    .fetch_all(pool)
    .await
    .map_err(|e| MoviesApiError::DatabaseError(e))
}

pub async fn get_movie_genre(pool: &PgPool, movie_id: i32) -> Result<Genre, MoviesApiError> {
    sqlx::query_as!(
        Genre,
        r#"
        SELECT * FROM genre
        WHERE id IN (SELECT genre_id FROM movie_genre WHERE movie_id = $1)
        "#,
        movie_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| MoviesApiError::DatabaseError(e))
}

pub async fn get_all_movies_ids(pool: &PgPool) -> Result<Vec<i32>, MoviesApiError> {
    sqlx::query_scalar!(
        r#"
    SELECT id from movie ORDER BY release_year;
    "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| MoviesApiError::DatabaseError(e))
}

pub async fn delete_movie(
    tx: &mut Transaction<'_, Postgres>,
    movie_id: i32,
) -> Result<(), MoviesApiError> {
    let res = sqlx::query!(
        r#"
        DELETE FROM movie WHERE id = $1
        "#,
        movie_id
    )
    .execute(&mut **tx)
    .await
    .map_err(|e| MoviesApiError::DatabaseError(e))?;

    if res.rows_affected() == 0 {
        return Err(MoviesApiError::MovieNotFound(movie_id));
    }

    // 2. Prune orphaned people
    sqlx::query!(
        r#"
        DELETE FROM person
        WHERE id NOT IN (
            SELECT person_id FROM movie_role
        )
        "#
    )
    .execute(&mut **tx)
    .await
    .map_err(|e| MoviesApiError::DatabaseError(e))?;

    // 3. Prune orphaned award_categories
    sqlx::query!(
        r#"
        DELETE FROM award_category
        WHERE id NOT IN (
            SELECT award_category_id FROM movie_award
            UNION
            SELECT award_category_id FROM movie_nomination
        )
        "#
    )
    .execute(&mut **tx)
    .await
    .map_err(|e| MoviesApiError::DatabaseError(e))?;

    // 4. Prune orphaned awards
    sqlx::query!(
        r#"
        DELETE FROM award
        WHERE id NOT IN (
            SELECT award_id FROM award_category
        )
        "#
    )
    .execute(&mut **tx)
    .await
    .map_err(|e| MoviesApiError::DatabaseError(e))?;

    // 5. Prune orphaned genre
    sqlx::query!(
        r#"
        DELETE FROM genre
        WHERE id NOT IN (
            SELECT genre_id FROM movie_genre
        )
        "#
    )
    .execute(&mut **tx)
    .await
    .map_err(|e| MoviesApiError::DatabaseError(e))?;

    Ok(())
}
