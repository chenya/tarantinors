use crate::interviews::api::errors::InterviewsApiError;
use axum::{Json, body::Body, extract::FromRequest, http::Request};
use serde::de::DeserializeOwned;
use validator::Validate;

pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = InterviewsApiError;

    async fn from_request(req: Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        // First extract JSON
        let Json(value) = Json::<T>::from_request(req, state).await.map_err(|err| {
            InterviewsApiError::InternalError(anyhow::anyhow!("JSON parsing failed: {}", err))
        })?;

        // Then validate
        value.validate()?;

        Ok(ValidatedJson(value))
    }
}
