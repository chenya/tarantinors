use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebAward {
    pub name: String,
    pub category: String,
    pub year: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebNomination {
    pub name: String,
    pub category: String,
    pub year: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebMovie {
    // pub id: usize,
    pub title: String,
    pub release_year: u16,
    pub genre: String,
    pub director: String,
    pub plot: String,
    pub runtime: u16,
    pub rating: f32,
    pub release_date: String,
    pub image_url: String,
    pub youtube_id: String,
    pub production_details: String,
    pub awards: Vec<WebAward>,
    pub nominations: Vec<WebNomination>,
    pub budget: String,
    pub producers: Vec<String>,
    pub actors: Vec<String>,
    pub writers: Vec<String>,
}

impl WebMovie {
    pub fn has_producers(&self) -> bool {
        !self.producers.is_empty()
    }

    pub fn has_actors(&self) -> bool {
        !self.actors.is_empty()
    }

    pub fn has_writers(&self) -> bool {
        !self.writers.is_empty()
    }
    pub fn has_nominations(&self) -> bool {
        !self.nominations.is_empty()
    }

    pub fn has_awards(&self) -> bool {
        !self.awards.is_empty()
    }
}
