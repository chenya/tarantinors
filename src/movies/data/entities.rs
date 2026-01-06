use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
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

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Genre {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Award {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct AwardCategory {
    pub id: i32,
    pub award_id: Option<i32>,
    pub category: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Junction table: movie ↔ person ↔ role
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct MovieRole {
    pub movie_id: i32,
    pub person_id: i32,
    pub role_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Junction table: movie ↔ genre
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct MovieGenre {
    pub movie_id: i32,
    pub genre_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Awards won by a movie
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct MovieAward {
    pub movie_id: i32,
    pub award_category_id: i32,
    pub year: i32,
    pub recipient: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Award nominations for a movie
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct MovieNomination {
    pub movie_id: i32,
    pub award_category_id: i32,
    pub year: i32,
    pub nominee: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Awards won by a movie
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovieAwardWon {
    pub name: String,
    pub category: String,
    pub year: i32,
    pub recipient: Option<String>,
}

/// Award nominations for a movie
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovieAwardNomination {
    pub name: String,
    pub category: String,
    pub year: i32,
    pub nominee: Option<String>,
}
