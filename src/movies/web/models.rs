use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MovieViewModel {
    pub id: i32,
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
    pub awards: Vec<MovieAwardViewModel>,
    pub nominations: Vec<MovieAwardNominationViewModel>,
}

impl MovieViewModel {
    pub fn has_directors(&self) -> bool {
        !self.directors.is_empty()
    }

    pub fn has_actors(&self) -> bool {
        !self.actors.is_empty()
    }

    pub fn has_producers(&self) -> bool {
        !self.producers.is_empty()
    }

    pub fn has_writers(&self) -> bool {
        !self.writers.is_empty()
    }

    pub fn has_awards(&self) -> bool {
        !self.awards.is_empty()
    }

    pub fn has_nominations(&self) -> bool {
        !self.nominations.is_empty()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MovieAwardViewModel {
    pub name: String,
    pub category: String,
    pub year: i32,
    pub recipient: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MovieAwardNominationViewModel {
    pub name: String,
    pub category: String,
    pub year: i32,
    pub nominee: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorViewModel {
    pub code: u16,
    pub message: String,
    pub details: Option<String>,
    pub show_suggestions: bool,
    pub title: String,
}
