use reqwest::{Client, StatusCode, header};

use std::time::Duration;
use tracing::{debug, error, info};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("HTTP error {status}:  {message}")]
    HttpError { status: StatusCode, message: String },
    #[error("Request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Not found")]
    NotFound,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Rate limited")]
    RateLimited,
}

pub type ClientResult<T> = Result<T, ClientError>;

pub struct HttpClient {
    client: Client,
    base_url: String,
}

impl HttpClient {
    pub fn new(base_url: String) -> ClientResult<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("TarantinorsCli/1.0")
            .http2_prior_knowledge() // Use HTTP/2 when possible
            .build()?;

        Ok(Self { client, base_url })
    }

    pub fn base_url(&self) -> &str {
        self.base_url.as_str()
    }

    pub async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> ClientResult<T> {
        let url = format!("{}{}", self.base_url, path);
        debug!("GET {}", url);

        let response = self.client.get(&url).send().await.map_err(|e| {
            error!("Request failed:  {}", e);
            ClientError::RequestFailed(e)
        })?;

        self.handle_response(response).await
    }

    pub async fn post<
        B: serde::ser::Serialize + std::fmt::Debug,
        T: serde::de::DeserializeOwned,
    >(
        &self,
        path: &str,
        body: &B,
    ) -> ClientResult<T> {
        let url = format!("{}{}", self.base_url, path);
        debug!("POST {} with body", url);

        let response = self
            .client
            .post(&url)
            .header(header::CONTENT_TYPE, "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                error!("Request failed: {}", e);
                ClientError::RequestFailed(e)
            })?;

        self.handle_response(response).await
    }

    pub async fn put<B: serde::ser::Serialize, T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> ClientResult<T> {
        let url = format!("{}{}", self.base_url, path);
        debug!("PUT {}", url);

        let response = self.client.put(&url).json(body).send().await?;

        self.handle_response(response).await
    }

    pub async fn delete<T: serde::de::DeserializeOwned>(&self, path: &str) -> ClientResult<T> {
        let url = format!("{}{}", self.base_url, path);
        debug!("DELETE {}", url);

        let response = self.client.delete(&url).send().await?;

        self.handle_response(response).await
    }

    async fn handle_response<T: serde::de::DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> ClientResult<T> {
        let status = response.status();

        match status {
            StatusCode::OK | StatusCode::CREATED | StatusCode::ACCEPTED => {
                let body = response.json::<T>().await?;
                info!("Response: OK");
                Ok(body)
            }
            StatusCode::NOT_FOUND => {
                error!("Not found");
                Err(ClientError::NotFound)
            }
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                error!("Unauthorized");
                Err(ClientError::Unauthorized)
            }
            StatusCode::TOO_MANY_REQUESTS => {
                error!("Rate limited");
                Err(ClientError::RateLimited)
            }
            _ => {
                let text = response.text().await.unwrap_or_default();
                error!("HTTP {}: {}", status, text);
                Err(ClientError::HttpError {
                    status,
                    message: text,
                })
            }
        }
    }
}
