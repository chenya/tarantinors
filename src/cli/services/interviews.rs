use crate::http_client::{ClientResult, HttpClient};
use serde::{Deserialize, Serialize};
use tarantino_rs::interviews::api::models::{
    CreateInterviewRequest, InterviewListResponse, InterviewMessage, InterviewResponse,
};
use tracing::error;

pub struct InterviewsService {
    http_client: HttpClient,
}

impl InterviewsService {
    pub fn new(base_url: String) -> ClientResult<Self> {
        let interviews_service_url = format!("{}/api/v1/interviews", base_url);

        let http_client = HttpClient::new(interviews_service_url)?;
        Ok(Self { http_client })
    }

    pub async fn get_interview(&self, id: i32) -> ClientResult<InterviewResponse> {
        let url_path = format!("/{}", id);
        self.http_client.get(&url_path).await
    }

    pub async fn list(&self) -> ClientResult<InterviewListResponse> {
        let url_path = "";
        self.http_client.get(url_path).await
    }

    pub async fn create_interview(
        &self,
        req: &CreateInterviewRequest,
    ) -> ClientResult<InterviewMessage> {
        let url_path = "";
        self.http_client.post(url_path, req).await
    }
    //
    pub async fn delete_interview(&self, id: i32) -> ClientResult<InterviewMessage> {
        let url_path = format!("/{}", id);
        self.http_client.delete(&url_path).await
    }
}
