use crate::movies::models::{CreateGenre, CreateMovie, CreatePerson, CreateAward, MovieInput, Person};

use serde::Deserialize;
use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};

use crate::movies::db::{
    create_actor, create_director, create_genre, create_movie, create_movie_genre, create_producer,
    create_writer, create_person, create_award, create_movie_award, create_movie_nomination, create_award_category
};
async fn init_dbpool() -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:1234@localhost:7777/tarantinodb".to_string());

    let db_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_connection_str)
        .await;

    db_pool
}

#[derive(Clone, Debug)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new() -> Self {
        let connection = match init_dbpool().await {
            Ok(pool) => pool,
            Err(e) => {
                panic!("Failed to create database pool: {}", e);
            }
        };
        Self { connection }
    }
    pub async fn insert_movie(&self, movie: MovieInput) -> Result<(), sqlx::Error> {
        let mut tx = self.connection.begin().await?;

        // 1. Movie
        let new_movie = CreateMovie::new(
            movie.title,
            movie.release_year,
            movie.plot,
            movie.runtime,
            movie.rating,
            movie.release_date,
            movie.image_url,
            movie.youtube_id,
            movie.budget,
            movie.production_details,
        );
        let movie_id = create_movie(&mut tx, new_movie).await?;

        // 2. Genre
        let new_genre = CreateGenre::new(movie.genre);
        let genre_id = create_genre(&mut tx, new_genre).await?;

        let movie_genre = create_movie_genre(&mut tx, movie_id, genre_id).await?;

        // 3. Roles
        // Directors
        for director in movie.directors {
            let new_person = CreatePerson::new(director);
            let person_id = create_person(&mut tx, new_person).await?;
            create_director(&mut tx, movie_id, person_id).await?;
        }

        // Producers
        for producer in movie.producers {
            let new_person = CreatePerson::new(producer);
            let person_id = create_person(&mut tx, new_person).await?;
            create_producer(&mut tx, movie_id, person_id).await?;
        }

        // Writers
        for writer in movie.writers {
            let new_person = CreatePerson::new(writer);
            let person_id = create_person(&mut tx, new_person).await?;
            create_writer(&mut tx, movie_id, person_id).await?;
        }

        // Actors
        for actor in movie.actors {
            let new_person = CreatePerson::new(actor);

            let person_id = create_person(&mut tx, new_person).await?;

            create_actor(&mut tx, movie_id, person_id)
                .await
                .expect(format!("{} {} Actor", movie_id, person_id).as_ref());
        }

        // 4. Awards
        for award in movie.awards {
            let new_award = CreateAward::new(award.name);
            let award_id = create_award(&mut tx, new_award).await?;
            let category_id = create_award_category(&mut tx, award_id, &award.category).await?;
            create_movie_award(&mut tx, movie_id, category_id, award.year, &award.recipient).await?;
        }

        // 5. Nominations
        for nom in movie.nominations {
            let new_award = CreateAward::new(nom.name);
            let award_id = create_award(&mut tx, new_award).await?;
            let category_id = create_award_category(&mut tx, award_id, &nom.category).await?;
            create_movie_nomination(&mut tx, movie_id, category_id, nom.year, &nom.nominee).await?;
        }

        tx.commit().await?;
        Ok(())
    }
}
