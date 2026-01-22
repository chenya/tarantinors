use crate::movies::api::errors::MoviesApiError;
use crate::movies::api::models::{
    CreateMovieRequest, MovieAwardNominationResponse, MovieAwardResponse, MovieResponse,
};
use crate::movies::data::repository::MovieRepository;
use futures::stream::{self, StreamExt};
use futures::TryStreamExt;
use sqlx::PgPool;
pub struct ApiService {
    repo: MovieRepository,
}

impl ApiService {
    pub fn new(pool: &PgPool) -> Self {
        Self {
            repo: MovieRepository::new(pool),
        }
    }

    pub async fn create_movie(&self, new_movie: CreateMovieRequest) -> Result<(), MoviesApiError> {
        let mut tx = self.repo.pool.begin().await?;

        // 1. Movie
        let movie = self
            .repo
            .create_movie(
                &mut tx,
                new_movie.title,
                new_movie.release_year,
                new_movie.plot,
                new_movie.runtime,
                new_movie.rating,
                new_movie.release_date,
                new_movie.image_url,
                new_movie.youtube_id,
                new_movie.budget,
                new_movie.production_details,
            )
            .await?;

        let movie_id = movie.id;

        // 2. Genre
        let new_genre = self.repo.create_genre(&mut tx, new_movie.genre).await?;
        let genre_id = new_genre.id;

        let _ = self
            .repo
            .create_movie_genre(&mut tx, movie_id, genre_id)
            .await?;

        // 3. Roles
        // Directors
        for director in new_movie.directors {
            let person = self.repo.create_person(&mut tx, director).await?;
            let person_id = person.id;
            self.repo
                .create_director(&mut tx, movie_id, person_id)
                .await?;
        }

        // Producers
        for producer in new_movie.producers {
            let person = self.repo.create_person(&mut tx, producer).await?;
            let person_id = person.id;
            self.repo
                .create_producer(&mut tx, movie_id, person_id)
                .await?;
        }

        // Writers
        for writer in new_movie.writers {
            let person = self.repo.create_person(&mut tx, writer).await?;
            let person_id = person.id;
            self.repo
                .create_writer(&mut tx, movie_id, person_id)
                .await?;
        }

        // Actors
        for actor in new_movie.actors {
            let person = self.repo.create_person(&mut tx, actor).await?;
            let person_id = person.id;

            self.repo.create_actor(&mut tx, movie_id, person_id).await?;
        }

        // 4. Awards
        for new_award in new_movie.awards {
            let award = self.repo.create_award(&mut tx, new_award.name).await?;
            let award_id = award.id;
            let award_category = self
                .repo
                .create_award_category(&mut tx, award_id, new_award.category)
                .await?;
            let category_id = award_category.id;
            self.repo
                .create_movie_award(
                    &mut tx,
                    movie_id,
                    category_id,
                    new_award.year,
                    new_award.recipient.as_str(),
                )
                .await?;
        }

        // 5. Nominations
        for new_nomination in new_movie.nominations {
            let award = self.repo.create_award(&mut tx, new_nomination.name).await?;
            let award_id = award.id;
            let award_category = self
                .repo
                .create_award_category(&mut tx, award_id, new_nomination.category)
                .await?;
            let category_id = award_category.id;
            self.repo
                .create_movie_nomination(
                    &mut tx,
                    movie_id,
                    category_id,
                    new_nomination.year,
                    new_nomination.nominee,
                )
                .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    pub async fn delete_movie(&self, movie_id: i32) -> Result<(), MoviesApiError> {
        self.movie_exists_guard(movie_id).await?;

        let mut tx = self.repo.pool.begin().await?;

        self.repo.delete_movie(&mut tx, movie_id).await?;

        tx.commit().await?;
        Ok(())
    }

    pub async fn get_movie(&self, movie_id: i32) -> Result<Option<MovieResponse>, MoviesApiError> {
        // 1. Movie
        let movie = match self.repo.get_movie_by_id(movie_id).await? {
            None => return Ok(None),
            Some(m) => m,
        };

        // 2-7. Fetch all data concurrently without existence checks
        let (directors, producers, writers, actors, awards, nominations, movie_genre) = tokio::join!(
            self.get_movie_directors_unchecked(movie_id),
            self.get_movie_producers_unchecked(movie_id),
            self.get_movie_writers_unchecked(movie_id),
            self.get_movie_actors_unchecked(movie_id),
            self.get_movie_awards_unchecked(movie_id),
            self.get_movie_nominations_unchecked(movie_id),
            self.repo.get_movie_genre(movie_id)
        );

        let movie_response = MovieResponse {
            title: movie.title,
            release_year: movie.release_year,
            genre: movie_genre?.name,
            plot: movie.plot,
            runtime: movie.runtime,
            rating: movie.rating,
            release_date: movie.release_date,
            image_url: movie.image_url,
            youtube_id: movie.youtube_id,
            budget: movie.budget,
            production_details: movie.production_details,
            directors: directors?,
            producers: producers?,
            actors: actors?,
            writers: writers?,
            awards: awards?,
            nominations: nominations?,
        };
        Ok(Some(movie_response))
    }

    pub async fn get_movies(&self) -> Result<Vec<MovieResponse>, MoviesApiError> {
        let movies_ids = self.repo.get_all_movies_ids().await?;
        let concurrency = movies_ids.len();

        let movies = stream::iter(movies_ids)
            .map(|movie_id| self.get_movie(movie_id))
            .buffered(concurrency)
            .try_collect::<Vec<Option<MovieResponse>>>()
            .await?
            .into_iter()
            .flatten()
            .collect();

        Ok(movies)
    }

    async fn movie_exists_guard(&self, movie_id: i32) -> Result<(), MoviesApiError> {
        self.repo
            .get_movie_by_id(movie_id)
            .await?
            .ok_or_else(|| MoviesApiError::MovieNotFound(movie_id))?;
        Ok(())
    }

    pub async fn get_movie_actors(&self, movie_id: i32) -> Result<Vec<String>, MoviesApiError> {
        self.movie_exists_guard(movie_id).await?;

        let actors = self.repo.get_movie_actors_names(movie_id).await?;
        Ok(actors)
    }

    pub async fn get_movie_directors(&self, movie_id: i32) -> Result<Vec<String>, MoviesApiError> {
        self.movie_exists_guard(movie_id).await?;

        let directors = self.repo.get_movie_directors_names(movie_id).await?;

        Ok(directors)
    }

    pub async fn get_movie_writers(&self, movie_id: i32) -> Result<Vec<String>, MoviesApiError> {
        self.movie_exists_guard(movie_id).await?;

        let writers = self.repo.get_movie_writers_names(movie_id).await?;

        Ok(writers)
    }

    pub async fn get_movie_producers(&self, movie_id: i32) -> Result<Vec<String>, MoviesApiError> {
        self.movie_exists_guard(movie_id).await?;

        let producers = self.repo.get_movie_producers_names(movie_id).await?;

        Ok(producers)
    }

    pub async fn get_movie_awards(
        &self,
        movie_id: i32,
    ) -> Result<Vec<MovieAwardResponse>, MoviesApiError> {
        self.movie_exists_guard(movie_id).await?;

        let awards = self
            .repo
            .get_movie_awards_won(movie_id)
            .await?
            .into_iter()
            .map(|a| MovieAwardResponse {
                name: a.name,
                category: a.category,
                year: a.year,
                recipient: a.recipient,
            })
            .collect();

        Ok(awards)
    }

    pub async fn get_movie_nominations(
        &self,
        movie_id: i32,
    ) -> Result<Vec<MovieAwardNominationResponse>, MoviesApiError> {
        self.movie_exists_guard(movie_id).await?;

        let nominations = self
            .repo
            .get_movie_awards_nominations(movie_id)
            .await?
            .into_iter()
            .map(|a| MovieAwardNominationResponse {
                name: a.name,
                category: a.category,
                year: a.year,
                nominee: a.nominee,
            })
            .collect();

        Ok(nominations)
    }

    async fn get_movie_actors_unchecked(
        &self,
        movie_id: i32,
    ) -> Result<Vec<String>, MoviesApiError> {
        let actors = self.repo.get_movie_actors_names(movie_id).await?;
        Ok(actors)
    }

    async fn get_movie_directors_unchecked(
        &self,
        movie_id: i32,
    ) -> Result<Vec<String>, MoviesApiError> {
        let directors = self.repo.get_movie_directors_names(movie_id).await?;

        Ok(directors)
    }

    async fn get_movie_writers_unchecked(
        &self,
        movie_id: i32,
    ) -> Result<Vec<String>, MoviesApiError> {
        let writers = self.repo.get_movie_writers_names(movie_id).await?;

        Ok(writers)
    }

    async fn get_movie_producers_unchecked(
        &self,
        movie_id: i32,
    ) -> Result<Vec<String>, MoviesApiError> {
        let producers = self.repo.get_movie_producers_names(movie_id).await?;

        Ok(producers)
    }

    async fn get_movie_awards_unchecked(
        &self,
        movie_id: i32,
    ) -> Result<Vec<MovieAwardResponse>, MoviesApiError> {
        let awards = self
            .repo
            .get_movie_awards_won(movie_id)
            .await?
            .into_iter()
            .map(|a| MovieAwardResponse {
                name: a.name,
                category: a.category,
                year: a.year,
                recipient: a.recipient,
            })
            .collect();

        Ok(awards)
    }

    async fn get_movie_nominations_unchecked(
        &self,
        movie_id: i32,
    ) -> Result<Vec<MovieAwardNominationResponse>, MoviesApiError> {
        let nominations = self
            .repo
            .get_movie_awards_nominations(movie_id)
            .await?
            .into_iter()
            .map(|a| MovieAwardNominationResponse {
                name: a.name,
                category: a.category,
                year: a.year,
                nominee: a.nominee,
            })
            .collect();

        Ok(nominations)
    }
}
