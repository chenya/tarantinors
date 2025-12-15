use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use validator::{Validate, ValidationError};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePerson {
    name: String,
}

impl CreatePerson {
    pub fn new(name: String) -> Self {
        Self { name }
    }
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MovieRolePerson {
    pub movie_id: i32,
    pub role: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MovieRole {
    pub movie_id: i32,
    pub person_id: i32,
    pub role_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGenre {
    name: String,
}

impl CreateGenre {
    pub fn new(name: String) -> Self {
        Self { name }
    }
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Genre {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Award {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CreateAward {
    name: String,
}

impl CreateAward {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Movie {
    pub id: i32,
    pub title: String,
    pub release_year: i32,
    pub plot: String,
    pub runtime: i32,
    pub rating: f32,
    pub release_date: NaiveDate,
    pub image_url: String,
    pub youtube_id: String,
    pub budget: String,
    pub production_details: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CreateMovie {
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
}
impl CreateMovie {
    pub fn new(
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
    ) -> Self {
        Self {
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
        }
    }

    pub fn title(&self) -> &str {
        self.title.as_ref()
    }
    pub fn release_year(&self) -> i32 {
        self.release_year
    }

    pub fn plot(&self) -> &str {
        self.plot.as_ref()
    }
    pub fn runtime(&self) -> i32 {
        self.runtime
    }
    pub fn rating(&self) -> f32 {
        self.rating
    }
    pub fn release_date(&self) -> NaiveDate {
        self.release_date
    }
    pub fn image_url(&self) -> &str {
        self.image_url.as_ref()
    }
    pub fn youtube_id(&self) -> &str {
        self.youtube_id.as_ref()
    }
    pub fn budget(&self) -> &str {
        self.budget.as_ref()
    }
    pub fn production_details(&self) -> &str {
        self.production_details.as_ref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MovieGenre {
    pub movie_id: i32,
    pub genre_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MovieAward {
    pub movie_id: i32,
    pub award_category_id: i32,
    pub year: i32,
    pub recipient: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AwardCategory {
    pub id: i32,
    pub award_id: i32,
    pub category: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MovieNomination {
    pub movie_id: i32,
    pub award_category_id: i32,
    pub year: i32,
    pub nominee: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct MovieAwardWon {
    pub name: String,
    pub category: String,
    pub year: i32,
    pub recipient: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct MovieAwardNomination {
    pub name: String,
    pub category: String,
    pub year: i32,
    pub nominee: String,
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
pub struct MovieInput {
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
    pub awards: Vec<MovieAwardWon>,
    pub nominations: Vec<MovieAwardNomination>,
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
pub struct MovieAwards {
    pub awards: Vec<MovieAwardWon>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MovieNominations {
    pub nominations: Vec<MovieAwardNomination>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MoviesMessage {
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Movies {
    pub movies: Vec<MovieInput>,
}
