use crate::http_client::{ClientResult, HttpClient};
use serde::{Deserialize, Serialize};
use tarantino_rs::quotes::api::models::{
    CreateQuoteRequest, QuoteListResponse, QuoteMessage, QuoteResponse,
};
use tracing::error;

pub struct QuotesService {
    http_client: HttpClient,
}

impl QuotesService {
    pub fn new(base_url: String) -> ClientResult<Self> {
        let quotes_service_url = format!("{}/api/v1/quotes", base_url);

        let http_client = HttpClient::new(quotes_service_url)?;
        Ok(Self { http_client })
    }

    pub async fn get_quote(&self, id: i32) -> ClientResult<QuoteResponse> {
        let url_path = format!("/{}", id);
        self.http_client.get(&url_path).await
    }

    pub async fn list_quotes(&self) -> ClientResult<QuoteListResponse> {
        let url_path = "";
        self.http_client.get(url_path).await
    }

    pub async fn create_quote(&self, req: &CreateQuoteRequest) -> ClientResult<QuoteMessage> {
        let url_path = "";
        self.http_client.post(url_path, req).await
    }

    pub async fn delete_quote(&self, id: i32) -> ClientResult<QuoteMessage> {
        let url_path = format!("/{}", id);
        self.http_client.delete(&url_path).await
    }
}
