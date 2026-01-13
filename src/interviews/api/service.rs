use crate::interviews::api::errors::InterviewsApiError;
use crate::interviews::api::models::{
    CreateInterviewRequest, InterviewListResponse, InterviewResponse,
};
use crate::interviews::data::repository::InterviewRepository;

use sqlx::PgPool;

pub struct ApiService {
    repo: InterviewRepository,
}

impl ApiService {
    pub fn new(pool: &PgPool) -> Self {
        Self {
            repo: InterviewRepository::new(pool),
        }
    }

    pub async fn create_interview(
        &self,
        new_interview: CreateInterviewRequest,
    ) -> Result<(), InterviewsApiError> {
        let mut tx = self.repo.pool.begin().await?;

        let _ = self
            .repo
            .create_interview(
                &mut tx,
                new_interview.title,
                new_interview.description,
                new_interview.youtube_id,
            )
            .await?;

        tx.commit().await?;

        Ok(())
    }

    pub async fn get_interview(
        &self,
        interview_id: i32,
    ) -> Result<Option<InterviewResponse>, InterviewsApiError> {
        let quote = self.repo.get_interview(interview_id).await?;

        match quote {
            None => Ok(None),
            Some(i) => Ok(Some(InterviewResponse {
                title: i.title,
                description: i.description,
                youtube_id: i.youtube_id,
            })),
        }
    }

    pub async fn get_interviews(&self) -> Result<InterviewListResponse, InterviewsApiError> {
        let interviews = self
            .repo
            .get_interviews()
            .await?
            .into_iter()
            .map(|i| InterviewResponse {
                title: i.title,
                description: i.description,
                youtube_id: i.youtube_id,
            })
            .collect();

        Ok(InterviewListResponse { interviews })
    }

    pub async fn delete_interview(&self, interview_id: i32) -> Result<(), InterviewsApiError> {
        let _ = self
            .get_interview(interview_id)
            .await?
            .ok_or(InterviewsApiError::NotFound(interview_id))?;

        let mut tx = self.repo.pool.begin().await?;

        self.repo.delete_interview(&mut tx, interview_id).await?;

        tx.commit().await?;

        Ok(())
    }
}
