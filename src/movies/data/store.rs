use crate::movies::api::errors::MoviesApiError;
use crate::movies::{api::models, data::db};
use sqlx::PgPool;

#[derive(Clone, Debug)]
pub struct MoviesStore {
    pub connection: PgPool,
}

impl MoviesStore {
    pub fn new(connection: PgPool) -> Self {
        Self { connection }
    }
    pub async fn insert_movie(&self, movie: models::MovieInput) -> Result<(), MoviesApiError> {
        let mut tx = self
            .connection
            .begin()
            .await
            .map_err(MoviesApiError::DatabaseError)?;

        // 1. Movie
        let new_movie = models::CreateMovie::new(
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
        let movie_id = db::create_movie(&mut tx, new_movie).await?;

        // 2. Genre
        let new_genre = models::CreateGenre::new(movie.genre);
        let genre_id = db::create_genre(&mut tx, new_genre).await?;

        let _ = db::create_movie_genre(&mut tx, movie_id, genre_id).await?;

        // 3. Roles
        // Directors
        for director in movie.directors {
            let new_person = models::CreatePerson::new(director);
            let person_id = db::create_person(&mut tx, new_person).await?;
            db::create_director(&mut tx, movie_id, person_id).await?;
        }

        // Producers
        for producer in movie.producers {
            let new_person = models::CreatePerson::new(producer);
            let person_id = db::create_person(&mut tx, new_person).await?;
            db::create_producer(&mut tx, movie_id, person_id).await?;
        }

        // Writers
        for writer in movie.writers {
            let new_person = models::CreatePerson::new(writer);
            let person_id = db::create_person(&mut tx, new_person).await?;
            db::create_writer(&mut tx, movie_id, person_id).await?;
        }

        // Actors
        for actor in movie.actors {
            let new_person = models::CreatePerson::new(actor);

            let person_id = db::create_person(&mut tx, new_person).await?;

            db::create_actor(&mut tx, movie_id, person_id).await?;
        }

        // 4. Awards
        for award in movie.awards {
            let new_award = models::CreateAward::new(award.name);
            let award_id = db::create_award(&mut tx, new_award).await?;
            let category_id = db::create_award_category(&mut tx, award_id, &award.category).await?;
            db::create_movie_award(&mut tx, movie_id, category_id, award.year, &award.recipient)
                .await?;
        }

        // 5. Nominations
        for nom in movie.nominations {
            let new_award = models::CreateAward::new(nom.name);
            let award_id = db::create_award(&mut tx, new_award).await?;
            let category_id = db::create_award_category(&mut tx, award_id, &nom.category).await?;
            db::create_movie_nomination(&mut tx, movie_id, category_id, nom.year, &nom.nominee)
                .await?;
        }

        tx.commit().await.map_err(MoviesApiError::DatabaseError)?;
        Ok(())
    }

    pub async fn is_movie_id_exists(&self, movie_id: i32) -> Result<bool, MoviesApiError> {
        db::is_movie_id_exists(&self.connection, movie_id).await
    }

    pub async fn get_movie(&self, movie_id: i32) -> Result<models::MovieInput, MoviesApiError> {
        // 1. Movie
        let movie = db::get_movie_by_id(&self.connection, movie_id).await?;

        // 2. Roles
        // Director
        let directors: Vec<String> =
            db::get_movie_directors_names(&self.connection, movie_id).await?;
        // Producers
        let producers = db::get_movie_producers_names(&self.connection, movie_id).await?;
        // Writers
        let writers = db::get_movie_writers_names(&self.connection, movie_id).await?;
        // Actors
        let actors = db::get_movie_actors_names(&self.connection, movie_id).await?;

        // 4. Awards
        let awards = db::get_movie_awards_won(&self.connection, movie_id).await?;
        // 5. Nominations
        let nominations = db::get_movie_awards_nominations(&self.connection, movie_id).await?;

        // 6. Genre
        let movie_genre = db::get_movie_genre(&self.connection, movie_id).await?;

        Ok(models::MovieInput {
            title: movie.title,
            release_year: movie.release_year,
            genre: movie_genre.name,
            plot: movie.plot,
            runtime: movie.runtime,
            rating: movie.rating,
            release_date: movie.release_date,
            image_url: movie.image_url,
            youtube_id: movie.youtube_id,
            budget: movie.budget,
            production_details: movie.production_details,
            directors,
            producers,
            actors,
            writers,
            awards,
            nominations,
        })
    }

    pub async fn get_movies(&self) -> Result<Vec<models::MovieInput>, MoviesApiError> {
        let movies_ids = db::get_all_movies_ids(&self.connection).await?;
        let mut movies = Vec::new();

        for movie_id in movies_ids {
            let movie = self.get_movie(movie_id).await?;
            movies.push(movie);
        }

        Ok(movies)
    }

    pub async fn delete_movie(&self, movie_id: i32) -> Result<(), MoviesApiError> {
        let mut tx = self.connection.begin().await?;
        let result = db::delete_movie(&mut tx, movie_id).await;

        match result {
            Ok(_) => tx
                .commit()
                .await
                .map_err(|e| MoviesApiError::DatabaseError(e)),
            Err(error) => {
                tx.rollback()
                    .await
                    .map_err(|e| MoviesApiError::DatabaseError(e))?;
                Err(error)
            }
        }
    }

    pub async fn get_movie_actors(&self, movie_id: i32) -> Result<Vec<String>, MoviesApiError> {
        db::get_movie_actors_names(&self.connection, movie_id).await
    }

    pub async fn get_movie_directors(&self, movie_id: i32) -> Result<Vec<String>, MoviesApiError> {
        db::get_movie_directors_names(&self.connection, movie_id).await
    }

    pub async fn get_movie_writers(&self, movie_id: i32) -> Result<Vec<String>, MoviesApiError> {
        db::get_movie_writers_names(&self.connection, movie_id).await
    }

    pub async fn get_movie_producers(&self, movie_id: i32) -> Result<Vec<String>, MoviesApiError> {
        db::get_movie_producers_names(&self.connection, movie_id).await
    }

    pub async fn get_movie_awards(
        &self,
        movie_id: i32,
    ) -> Result<Vec<models::MovieAwardWon>, MoviesApiError> {
        db::get_movie_awards_won(&self.connection, movie_id).await
    }

    pub async fn get_movie_nominations(
        &self,
        movie_id: i32,
    ) -> Result<Vec<models::MovieAwardNomination>, MoviesApiError> {
        db::get_movie_awards_nominations(&self.connection, movie_id).await
    }
}
