use crate::http_client::{ClientResult, HttpClient};
use serde::{Deserialize, Serialize};
use tarantino_rs::movies::api::models::{
    CreateMovieRequest, MovieListResponse, MovieResponse, MoviesMessage,
};
use tracing::error;

pub struct MoviesService {
    http_client: HttpClient,
}

impl MoviesService {
    pub fn new(base_url: String) -> ClientResult<Self> {
        let movies_service_url = format!("{}/api/v1/movies", base_url);

        let http_client = HttpClient::new(movies_service_url)?;
        Ok(Self { http_client })
    }

    pub async fn get_movie(&self, id: i32) -> ClientResult<MovieResponse> {
        let url_path = format!("/{}", id);
        self.http_client.get(&url_path).await
    }

    pub async fn list(&self) -> ClientResult<MovieListResponse> {
        let url_path = "";
        self.http_client.get(url_path).await
    }

    pub async fn create_movie(&self, req: &CreateMovieRequest) -> ClientResult<MoviesMessage> {
        let url_path = "";
        self.http_client.post(url_path, req).await
    }
    //
    pub async fn delete_movie(&self, id: i32) -> ClientResult<MoviesMessage> {
        let url_path = format!("/{}", id);
        self.http_client.delete(&url_path).await
    }
}
