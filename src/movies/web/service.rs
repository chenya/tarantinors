use crate::movies::data::repository::MovieRepository;
use crate::movies::web::models::{
    MovieAwardNominationViewModel, MovieAwardViewModel, MovieViewModel,
};

use futures::TryStreamExt;
use futures::stream::{self, StreamExt};
use sqlx::PgPool;

pub struct WebService {
    repo: MovieRepository,
}

impl WebService {
    pub fn new(pool: &PgPool) -> Self {
        Self {
            repo: MovieRepository::new(pool),
        }
    }
    pub async fn get_movie(&self, movie_id: i32) -> Result<Option<MovieViewModel>, sqlx::Error> {
        // 1. Movie
        let movie = match self.repo.get_movie_by_id(movie_id).await? {
            None => return Ok(None),
            Some(m) => m,
        };

        // 2. Roles
        // Director
        let directors = self.get_movie_directors(movie_id).await?;
        // Producers
        let producers = self.get_movie_producers(movie_id).await?;
        // Writers
        let writers = self.get_movie_writers(movie_id).await?;
        // Actors
        let actors = self.get_movie_actors(movie_id).await?;

        // 4. Awards
        let awards = self.get_movie_awards(movie_id).await?;

        // 5. Nominations
        let nominations = self.get_movie_nominations(movie_id).await?;

        // 6. Genre
        let movie_genre = self.repo.get_movie_genre(movie_id).await?;

        let movie_view_model = MovieViewModel {
            id: movie_id,
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
        };
        Ok(Some(movie_view_model))
    }

    pub async fn get_movies(&self) -> Result<Vec<MovieViewModel>, sqlx::Error> {
        let movies_ids = self.repo.get_all_movies_ids().await?;
        let concurrency = movies_ids.len();

        let movies = stream::iter(movies_ids)
            .map(|movie_id| self.get_movie(movie_id))
            .buffered(concurrency)
            .try_collect::<Vec<Option<MovieViewModel>>>()
            .await?
            .into_iter()
            .flatten()
            .collect();

        Ok(movies)
    }

    pub async fn get_movie_actors(&self, movie_id: i32) -> Result<Vec<String>, sqlx::Error> {
        self.repo.get_movie_actors_names(movie_id).await
    }

    pub async fn get_movie_directors(&self, movie_id: i32) -> Result<Vec<String>, sqlx::Error> {
        self.repo.get_movie_directors_names(movie_id).await
    }

    pub async fn get_movie_writers(&self, movie_id: i32) -> Result<Vec<String>, sqlx::Error> {
        self.repo.get_movie_writers_names(movie_id).await
    }

    pub async fn get_movie_producers(&self, movie_id: i32) -> Result<Vec<String>, sqlx::Error> {
        self.repo.get_movie_producers_names(movie_id).await
    }

    pub async fn get_movie_awards(
        &self,
        movie_id: i32,
    ) -> Result<Vec<MovieAwardViewModel>, sqlx::Error> {
        let awards = self
            .repo
            .get_movie_awards_won(movie_id)
            .await?
            .iter()
            .map(|a| MovieAwardViewModel {
                name: a.name.clone(),
                category: a.category.clone(),
                year: a.year,
                recipient: a.recipient.clone(),
            })
            .collect();

        Ok(awards)
    }

    pub async fn get_movie_nominations(
        &self,
        movie_id: i32,
    ) -> Result<Vec<MovieAwardNominationViewModel>, sqlx::Error> {
        let nominations = self
            .repo
            .get_movie_awards_nominations(movie_id)
            .await?
            .iter()
            .map(|a| MovieAwardNominationViewModel {
                name: a.name.clone(),
                category: a.category.clone(),
                year: a.year,
                nominee: a.nominee.clone(),
            })
            .collect();

        Ok(nominations)
    }
}
