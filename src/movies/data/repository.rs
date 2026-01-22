use chrono::{NaiveDate, NaiveDateTime};
use sqlx::{PgPool, Postgres, Transaction};

use crate::movies::data::entities::{
    Award, AwardCategory, Genre, Movie, MovieAward, MovieAwardNomination, MovieAwardWon,
    MovieGenre, MovieNomination, MovieRole, Person,
};

pub enum PersonRole {
    Actor,
    Director,
    Producer,
    Writer,
}
impl std::fmt::Display for PersonRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            PersonRole::Actor => "Actor",
            PersonRole::Director => "Director",
            PersonRole::Producer => "Producer",
            PersonRole::Writer => "Writer",
        };
        write!(f, "{}", s)
    }
}

pub struct MovieRepository {
    pub pool: PgPool,
}

impl MovieRepository {
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn create_person(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        person_name: String,
    ) -> Result<Person, sqlx::Error> {
        sqlx::query_as!(
            Person,
            r#"
            INSERT INTO person (name ) VALUES ($1)
            ON CONFLICT (name) DO UPDATE SET name = EXCLUDED.name
            RETURNING *
            "#,
            person_name
        )
        .fetch_one(&mut **tx)
        .await
    }

    pub async fn create_genre(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        genre_name: String,
    ) -> Result<Genre, sqlx::Error> {
        sqlx::query_as!(
            Genre,
            r#"
            INSERT INTO genre (name ) VALUES ($1)
            ON CONFLICT (name) DO UPDATE SET name = EXCLUDED.name
            RETURNING *
            "#,
            genre_name
        )
        .fetch_one(&mut **tx)
        .await
    }

    pub async fn create_award(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        award_name: String,
    ) -> Result<Award, sqlx::Error> {
        sqlx::query_as!(
            Award,
            r#"
            INSERT INTO award (name) VALUES  ($1)
            ON CONFLICT (name) DO UPDATE SET name = EXCLUDED.name
            RETURNING *
            "#,
            award_name
        )
        .fetch_one(&mut **tx)
        .await
    }

    pub async fn create_movie_role(
        &self,
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
            RETURNING *
            "#,
            movie_id,
            person_id,
            role.to_string()
        )
        .fetch_one(&mut **tx)
        .await
    }

    pub async fn create_actor(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        movie_id: i32,
        person_id: i32,
    ) -> Result<MovieRole, sqlx::Error> {
        self.create_movie_role(tx, movie_id, person_id, PersonRole::Actor)
            .await
    }

    pub async fn create_writer(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        movie_id: i32,
        person_id: i32,
    ) -> Result<MovieRole, sqlx::Error> {
        self.create_movie_role(tx, movie_id, person_id, PersonRole::Writer)
            .await
    }

    pub async fn create_producer(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        movie_id: i32,
        person_id: i32,
    ) -> Result<MovieRole, sqlx::Error> {
        self.create_movie_role(tx, movie_id, person_id, PersonRole::Producer)
            .await
    }

    pub async fn create_director(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        movie_id: i32,
        person_id: i32,
    ) -> Result<MovieRole, sqlx::Error> {
        self.create_movie_role(tx, movie_id, person_id, PersonRole::Director)
            .await
    }

    pub async fn create_movie(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        title: String,
        release_year: i32,
        plot: String,
        runtime: i32,
        rating: f32,
        release_date: NaiveDate,
        image_url: String,
        youtube_id: String,
        budget: String,
        production_details: String,
    ) -> Result<Movie, sqlx::Error> {
        sqlx::query_as!(
            Movie,
            r#"
        INSERT INTO movie (
            title, release_year,  plot, runtime, rating,
            release_date, image_url, youtube_id, budget, production_details
        ) VALUES (
            $1,$2,$3,$4,$5,$6,$7,$8,$9,$10
        )
        ON CONFLICT (title) DO UPDATE SET title = EXCLUDED.title
        RETURNING *
        "#,
            title,
            release_year,
            plot,
            runtime,
            rating,
            release_date,
            image_url,
            youtube_id,
            budget,
            production_details,
        )
        .fetch_one(&mut **tx)
        .await
    }

    pub async fn create_movie_genre(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        movie_id: i32,
        genre_id: i32,
    ) -> Result<MovieGenre, sqlx::Error> {
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
    }

    pub async fn create_award_category(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        award_id: i32,
        category_name: String,
    ) -> Result<AwardCategory, sqlx::Error> {
        sqlx::query_as!(
            AwardCategory,
            r#"
            INSERT INTO award_category (award_id, category) VALUES ($1, $2)
            ON CONFLICT (award_id, category) DO UPDATE SET category = EXCLUDED.category
            RETURNING *
            "#,
            award_id,
            category_name,
        )
        .fetch_one(&mut **tx)
        .await
    }

    pub async fn create_movie_award(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        movie_id: i32,
        award_category_id: i32,
        year: i32,
        recipient: &str,
    ) -> Result<MovieAward, sqlx::Error> {
        sqlx::query_as!(
            MovieAward,
            r#"
            INSERT INTO movie_award (movie_id, award_category_id, year, recipient)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT DO NOTHING
            RETURNING *
            "#,
            movie_id,
            award_category_id,
            year,
            recipient
        )
        .fetch_one(&mut **tx)
        .await
    }

    pub async fn create_movie_nomination(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        movie_id: i32,
        award_category_id: i32,
        year: i32,
        nominee: String,
    ) -> Result<MovieNomination, sqlx::Error> {
        sqlx::query_as!(
            MovieNomination,
            r#"
            INSERT INTO movie_nomination (movie_id, award_category_id, year, nominee)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT DO NOTHING
            RETURNING *
            "#,
            movie_id,
            award_category_id,
            year,
            nominee
        )
        .fetch_one(&mut **tx)
        .await
    }

    pub async fn delete_movie(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        movie_id: i32,
    ) -> Result<(), sqlx::Error> {
        let res = sqlx::query!(
            r#"
        DELETE FROM movie WHERE id = $1
        "#,
            movie_id
        )
        .execute(&mut **tx)
        .await?;

        // if res.rows_affected() == 0 {
        //     return Err(MoviesApiError::MovieNotFound(movie_id));
        // }

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
        .await?;

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
        .await?;

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
        .await?;

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
        .await?;

        Ok(())
    }

    pub async fn get_movie_by_id(&self, movie_id: i32) -> Result<Option<Movie>, sqlx::Error> {
        sqlx::query_as!(
            Movie,
            r#"
            SELECT * FROM movie WHERE id = $1
            "#,
            movie_id
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn get_persons_by_role(
        &self,
        movie_id: i32,
        role: PersonRole,
    ) -> Result<Vec<Person>, sqlx::Error> {
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
        .fetch_all(&self.pool)
        .await?;

        Ok(persons)
    }

    async fn get_persons_names_by_role(
        &self,

        movie_id: i32,
        role: PersonRole,
    ) -> Result<Vec<String>, sqlx::Error> {
        let persons_names = self
            .get_persons_by_role(movie_id, role)
            .await?
            .into_iter()
            .map(|p| p.name)
            .collect();
        Ok(persons_names)
    }

    pub async fn get_movie_actors_names(&self, movie_id: i32) -> Result<Vec<String>, sqlx::Error> {
        self.get_persons_names_by_role(movie_id, PersonRole::Actor)
            .await
    }
    pub async fn get_movie_directors_names(
        &self,
        movie_id: i32,
    ) -> Result<Vec<String>, sqlx::Error> {
        self.get_persons_names_by_role(movie_id, PersonRole::Director)
            .await
    }
    pub async fn get_movie_producers_names(
        &self,
        movie_id: i32,
    ) -> Result<Vec<String>, sqlx::Error> {
        self.get_persons_names_by_role(movie_id, PersonRole::Producer)
            .await
    }
    pub async fn get_movie_writers_names(&self, movie_id: i32) -> Result<Vec<String>, sqlx::Error> {
        self.get_persons_names_by_role(movie_id, PersonRole::Writer)
            .await
    }

    pub async fn get_movie_awards_won(
        &self,
        movie_id: i32,
    ) -> Result<Vec<MovieAwardWon>, sqlx::Error> {
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
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_movie_awards_nominations(
        &self,
        movie_id: i32,
    ) -> Result<Vec<MovieAwardNomination>, sqlx::Error> {
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
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_movie_genre(&self, movie_id: i32) -> Result<Genre, sqlx::Error> {
        sqlx::query_as!(
            Genre,
            r#"
            SELECT * FROM genre
            WHERE id IN (SELECT genre_id FROM movie_genre WHERE movie_id = $1)
            "#,
            movie_id
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_all_movies_ids(&self) -> Result<Vec<i32>, sqlx::Error> {
        sqlx::query_scalar!(
            r#"
        SELECT id from movie ORDER BY release_year;
        "#,
        )
        .fetch_all(&self.pool)
        .await
    }
}
