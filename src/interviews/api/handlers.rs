use crate::interviews::{
    api::{
        errors::{InterviewApiErrorResponse, InterviewsApiError},
        extractors::ValidatedJson,
        models::{
            CreateInterviewRequest, InterviewListResponse, InterviewMessage, InterviewResponse,
        },
        service::ApiService,
    },

};

use axum::Extension;
use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::{error, info, instrument};
use utoipa::OpenApi;
use crate::store::Store;


/// Create a new Interview
#[utoipa::path(
    post,
    path = "/interviews",
    request_body = CreateInterviewRequest,
    responses(
        (status = 201, description = "Interview Created", body = CreateInterviewRequest),
        (status = 400, description = "Request Validation Error", body = InterviewApiErrorResponse),
        (status = 500, description = "Internal server error", body = InterviewApiErrorResponse)
    ),
    tag = "Interviews API"
)]
#[instrument]
pub async fn add_new_interview(
    Extension(store): Extension<Store>,
    ValidatedJson(new_interview): ValidatedJson<CreateInterviewRequest>,
) -> Result<impl IntoResponse, InterviewsApiError> {
    let service = ApiService::new(&store.connection);
    let interview_title = new_interview.title.clone();
    let _ = service.create_interview(new_interview).await?;

    let message = format!("Interview '{}' added ", interview_title);
    info!(%message);
    Ok((StatusCode::CREATED, Json(InterviewMessage { message })))
}

/// Get interview by ID
#[utoipa::path(
    get,
    path = "/interviews/{interview_id}",
    responses(
        (status = 200, description = "Interview found", body = InterviewResponse),
        (status = 404, description = "Interview not found", body = InterviewApiErrorResponse),
        (status = 500, description = "Database server error", body = InterviewApiErrorResponse)
    ),
    tag = "Interviews API"
)]
#[instrument]
pub async fn get_interview(
    Extension(store): Extension<Store>,
    Path(interview_id): Path<i32>,
) -> Result<impl IntoResponse, InterviewsApiError> {
    // let movie = store.get_movie(movie_id).await?;

    let service = ApiService::new(&store.connection);

    let interview = service
        .get_interview(interview_id)
        .await?
        .ok_or(InterviewsApiError::NotFound(interview_id))?;

    info!("queried interview {interview_id}");
    Ok(Json(interview))
}

/// Get list of interviews
#[utoipa::path(
    get,
    path = "/interviews",
    responses(
        (status = 200, description = "List of Interviews", body = InterviewListResponse),
        (status = 500, description = "Database server error", body = InterviewApiErrorResponse)
    ),
    tag = "Interviews API"
)]
#[instrument]
pub async fn get_interviews(
    Extension(store): Extension<Store>,
) -> Result<impl IntoResponse, InterviewsApiError> {
    let service = ApiService::new(&store.connection);

    let interviews = service.get_interviews().await?;

    info!("queried all interviews");
    Ok(Json(interviews))
}

/// Delete interview by ID
#[utoipa::path(
    delete,
    path = "/interviews/{interview_id}",
    responses(
        (status = 200, description = "Interview Deleted", body = InterviewMessage),
        (status = 404, description = "Interview not found", body = InterviewApiErrorResponse),
        (status = 500, description = "Database server error", body = InterviewApiErrorResponse)
    ),
    tag = "Interviews API"
)]
#[instrument]
pub async fn remove_interview(
    Extension(store): Extension<Store>,
    Path(interview_id): Path<i32>,
) -> Result<impl IntoResponse, InterviewsApiError> {
    let service = ApiService::new(&store.connection);

    let _ = service.delete_interview(interview_id).await?;

    let message = format!("Quote {interview_id} deleted");

    info!(%message);
    Ok(Json(InterviewMessage { message }))
}

#[derive(OpenApi)]
#[openapi(
    paths(add_new_interview, get_interview, get_interviews, remove_interview,),
    components(schemas()),
    modifiers()
)]
pub struct InterviewsApiDoc;
