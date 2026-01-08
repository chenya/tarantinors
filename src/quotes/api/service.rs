use crate::quotes::api::models::{CreateQuoteRequest, QuoteListResponse, QuoteResponse};
use crate::quotes::data::repository::QuoteRepository;
use futures::TryStreamExt;
use futures::stream::{self, StreamExt};
use sqlx::PgPool;

pub struct ApiService {
    repo: QuoteRepository,
}

impl ApiService {
    pub fn new(pool: &PgPool) -> Self {
        Self {
            repo: QuoteRepository::new(pool),
        }
    }

    pub async fn create_quote(&self, new_quote: CreateQuoteRequest) -> Result<(), sqlx::Error> {
        let mut tx = self.repo.pool.begin().await?;
        let _ = self.repo.create_quote(&mut tx, new_quote.text).await?;

        tx.commit().await?;

        Ok(())
    }

    pub async fn get_quote(&self, quote_id: i32) -> Result<Option<QuoteResponse>, sqlx::Error> {
        let quote = self.repo.get_quote(quote_id).await?;
        match quote {
            None => Ok(None),
            Some(q) => Ok(Some(QuoteResponse { text: q.text })),
        }
    }

    pub async fn get_quotes(&self) -> Result<QuoteListResponse, sqlx::Error> {
        let quotes = self
            .repo
            .get_quotes()
            .await?
            .iter()
            .map(|q| QuoteResponse {
                text: q.text.clone(),
            })
            .collect();

        Ok(QuoteListResponse { quotes })
    }

    pub async fn delete_quote(&self, quote_id: i32) -> Result<(), sqlx::Error> {
        let mut tx = self.repo.pool.begin().await?;

        let _ = self.repo.delete_quote(&mut tx, quote_id).await?;

        tx.commit().await?;

        Ok(())
    }
}
