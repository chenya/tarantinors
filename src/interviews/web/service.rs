use crate::interviews::data::repository::InterviewRepository;
// use crate::movies::web::errors::MoviesWebError;
use crate::interviews::web::models::InterviewViewModel;

use sqlx::PgPool;

pub struct WebService {
    repo: InterviewRepository,
}

impl WebService {
    pub fn new(pool: &PgPool) -> Self {
        Self {
            repo: InterviewRepository::new(pool),
        }
    }

    pub async fn get_interviews(&self) -> Vec<InterviewViewModel> {
        let interviews = self
            .repo
            .get_interviews()
            .await
            .unwrap()
            .into_iter()
            .map(|i| InterviewViewModel {
                title: i.title,
                description: i.description,
                youtube_id: i.youtube_id,
            })
            .collect();

        interviews
    }
}
