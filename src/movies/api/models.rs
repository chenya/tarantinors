use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::{Validate, ValidationError};

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct CreateMovieAwardRequest {
    pub name: String,
    pub category: String,
    pub year: i32,
    pub recipient: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct CreateMovieAwardNominationRequest {
    pub name: String,
    pub category: String,
    pub year: i32,
    pub nominee: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct MovieAwardResponse {
    pub name: String,
    pub category: String,
    pub year: i32,
    pub recipient: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct MovieAwardNominationResponse {
    pub name: String,
    pub category: String,
    pub year: i32,
    pub nominee: Option<String>,
}

fn validate_date(date: &NaiveDate) -> Result<(), ValidationError> {
    let today = chrono::Local::now().naive_local().date();
    if *date > today {
        let mut error = ValidationError::new("date_must_be_past");
        error.message = Some("Date must be in the past".into());
        error.add_param("provided_date".into(), &date.to_string());
        error.add_param("today".into(), &today.to_string());
        return Err(error);
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateMovieRequest {
    #[validate(length(min = 1, message = "Title cannot be empty"))]
    pub title: String,
    #[validate(range(min = 1963, message = "Release year must be after 1963"))]
    pub release_year: i32,
    #[validate(length(min = 1, message = "Genre cannot be empty"))]
    pub genre: String,
    #[validate(length(min = 1, message = "Plot cannot be empty"))]
    pub plot: String,
    #[validate(range(min = 1, message = "Runtime must be positive"))]
    pub runtime: i32,
    #[validate(range(min = 0.0, max = 10.0, message = "Rating must be between 0 and 10"))]
    pub rating: f32,
    #[validate(custom(
        function = "validate_date",
        message = "Release date must be in the past"
    ))]
    pub release_date: NaiveDate,
    #[validate(url(message = "Image URL must be a valid URL"))]
    pub image_url: String,
    #[validate(length(min = 1, message = "YouTube ID cannot be empty"))]
    pub youtube_id: String,
    #[validate(length(min = 1, message = "Production details cannot be empty"))]
    pub production_details: String,
    #[validate(length(min = 1, message = "Budget cannot be empty"))]
    pub budget: String,
    #[validate(length(min = 1, message = "Directors cannot be empty"))]
    pub directors: Vec<String>,
    #[validate(length(min = 1, message = "Producers cannot be empty"))]
    pub producers: Vec<String>,
    #[validate(length(min = 1, message = "Actors cannot be empty"))]
    pub actors: Vec<String>,
    #[validate(length(min = 1, message = "Writers cannot be empty"))]
    pub writers: Vec<String>,
    pub awards: Vec<CreateMovieAwardRequest>,
    pub nominations: Vec<CreateMovieAwardNominationRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MovieResponse {
    pub title: String,
    pub release_year: i32,
    pub genre: String,
    pub plot: String,
    pub runtime: i32,
    pub rating: f32,
    pub release_date: NaiveDate,
    pub image_url: String,
    pub youtube_id: String,
    pub production_details: String,
    pub budget: String,
    pub directors: Vec<String>,
    pub producers: Vec<String>,
    pub actors: Vec<String>,
    pub writers: Vec<String>,
    pub awards: Vec<MovieAwardResponse>,
    pub nominations: Vec<MovieAwardNominationResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Actors {
    pub actors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Directors {
    pub directors: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Producers {
    pub producers: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Writers {
    pub writers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MovieAwardsResponse {
    pub awards: Vec<MovieAwardResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MovieNominationsResponse {
    pub nominations: Vec<MovieAwardNominationResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MoviesMessage {
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Movies {
    pub movies: Vec<MovieResponse>,
}
