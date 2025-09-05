use crate::movies::db::{
    create_actor, create_award, create_award_category, create_director, create_genre, create_movie,
    create_movie_award, create_movie_genre, create_movie_nomination, create_person,
    create_producer, create_writer, delete_movie, get_all_movies_ids, get_movie_actors_names,
    get_movie_awards_nominations, get_movie_awards_won, get_movie_by_id, get_movie_directors_names,
    get_movie_genre, get_movie_producers_names, get_movie_writers_names,
};
use crate::movies::models::{
    CreateAward, CreateGenre, CreateMovie, CreatePerson, Movie, MovieAwardNomination,
    MovieAwardWon, MovieInput,
};
use sqlx::PgPool;

#[derive(Clone, Debug)]
pub struct MoviesStore {
    pub connection: PgPool,
}

impl MoviesStore {
    pub fn new(connection: PgPool) -> Self {
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
            create_movie_award(&mut tx, movie_id, category_id, award.year, &award.recipient)
                .await?;
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

    pub async fn get_movie(&self, movie_id: i32) -> Result<Option<MovieInput>, sqlx::Error> {
        // 1. Movie
        let movie = match get_movie_by_id(&self.connection, movie_id).await? {
            Some(m) => m,
            None => return Ok(None),
        };

        // 2. Roles
        // Director
        let directors: Vec<String> = get_movie_directors_names(&self.connection, movie_id).await?;
        // Producers
        let producers = get_movie_producers_names(&self.connection, movie_id).await?;
        // Writers
        let writers = get_movie_writers_names(&self.connection, movie_id).await?;
        // Actors
        let actors = get_movie_actors_names(&self.connection, movie_id).await?;

        // 4. Awards
        let awards = get_movie_awards_won(&self.connection, movie_id).await?;
        // 5. Nominations
        let nominations = get_movie_awards_nominations(&self.connection, movie_id).await?;

        // 6. Genre
        let movie_genre = get_movie_genre(&self.connection, movie_id).await?;

        Ok(Some(MovieInput {
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
        }))
    }

    pub async fn get_movies(&self) -> Result<Vec<MovieInput>, sqlx::Error> {
        let movies_ids = get_all_movies_ids(&self.connection).await?;
        let mut movies = Vec::new();

        for movie_id in movies_ids {
            match self.get_movie(movie_id).await {
                Ok(Some(movie)) => movies.push(movie),
                Ok(None) => {}
                Err(e) => return Err(e),
            }
        }

        Ok(movies)
    }

    pub async fn delete_movie(&self, movie_id: i32) -> Result<bool, sqlx::Error> {
        let mut tx = self.connection.begin().await?;
        let result = delete_movie(&mut tx, movie_id).await?;

        match result {
            true => tx.commit().await?,
            false => tx.rollback().await?,
        }

        Ok(result)
    }

    pub async fn get_movie_actors(&self, movie_id: i32) -> Result<Vec<String>, sqlx::Error> {
        get_movie_actors_names(&self.connection, movie_id).await
    }

    pub async fn get_movie_directors(&self, movie_id: i32) -> Result<Vec<String>, sqlx::Error> {
        get_movie_directors_names(&self.connection, movie_id).await
    }

    pub async fn get_movie_writers(&self, movie_id: i32) -> Result<Vec<String>, sqlx::Error> {
        get_movie_writers_names(&self.connection, movie_id).await
    }

    pub async fn get_movie_producers(&self, movie_id: i32) -> Result<Vec<String>, sqlx::Error> {
        get_movie_producers_names(&self.connection, movie_id).await
    }

    pub async fn get_movie_awards(&self, movie_id: i32) -> Result<Vec<MovieAwardWon>, sqlx::Error> {
        get_movie_awards_won(&self.connection, movie_id).await
    }

    pub async fn get_movie_nominations(
        &self,
        movie_id: i32,
    ) -> Result<Vec<MovieAwardNomination>, sqlx::Error> {
        get_movie_awards_nominations(&self.connection, movie_id).await
    }
}
