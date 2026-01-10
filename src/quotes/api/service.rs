use crate::quotes::api::errors::QuotesApiError;
use crate::quotes::api::models::{CreateQuoteRequest, QuoteListResponse, QuoteResponse};
use crate::quotes::data::repository::QuoteRepository;

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

    pub async fn create_quote(&self, new_quote: CreateQuoteRequest) -> Result<(), QuotesApiError> {
        let mut tx = self
            .repo
            .pool
            .begin()
            .await
            .map_err(|e| QuotesApiError::DatabaseError(e))?;

        let _ = self
            .repo
            .create_quote(&mut tx, new_quote.text)
            .await
            .map_err(|e| QuotesApiError::DatabaseError(e))?;

        tx.commit()
            .await
            .map_err(|e| QuotesApiError::DatabaseError(e))?;

        Ok(())
    }

    pub async fn get_quote(&self, quote_id: i32) -> Result<Option<QuoteResponse>, QuotesApiError> {
        let quote = self
            .repo
            .get_quote(quote_id)
            .await
            .map_err(|e| QuotesApiError::DatabaseError(e))?;

        match quote {
            None => Ok(None),
            Some(q) => Ok(Some(QuoteResponse { text: q.text })),
        }
    }

    pub async fn get_quotes(&self) -> Result<QuoteListResponse, QuotesApiError> {
        let quotes = self
            .repo
            .get_quotes()
            .await
            .map_err(|e| QuotesApiError::DatabaseError(e))?
            .iter()
            .map(|q| QuoteResponse {
                text: q.text.clone(),
            })
            .collect();

        Ok(QuoteListResponse { quotes })
    }

    pub async fn delete_quote(&self, quote_id: i32) -> Result<(), QuotesApiError> {
        let mut tx = self
            .repo
            .pool
            .begin()
            .await
            .map_err(|e| QuotesApiError::DatabaseError(e))?;

        let _ = self
            .repo
            .delete_quote(&mut tx, quote_id)
            .await
            .map_err(|e| QuotesApiError::DatabaseError(e))?;

        tx.commit()
            .await
            .map_err(|e| QuotesApiError::DatabaseError(e))?;

        Ok(())
    }
}
