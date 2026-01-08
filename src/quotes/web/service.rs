use crate::quotes::data::repository::QuoteRepository;
// use crate::movies::web::errors::MoviesWebError;
use crate::quotes::web::models::QuoteViewModel;

use sqlx::PgPool;

pub struct WebService {
    repo: QuoteRepository,
}

impl WebService {
    pub fn new(pool: &PgPool) -> Self {
        Self {
            repo: QuoteRepository::new(pool),
        }
    }

    pub async fn get_quotes(&self) -> Vec<QuoteViewModel> {
        let quotes = self
            .repo
            .get_quotes()
            .await
            .unwrap()
            .iter()
            .map(|q| QuoteViewModel {
                text: q.text.clone(),
            })
            .collect();

        quotes
    }
}
