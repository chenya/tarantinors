use crate::interviews::{
    data::store::InterviewStore,
    web::{
        models::InterviewViewModel,
        service::WebService,
        templates::{HtmxInterviewsTemplate, InterviewsTemplate},
    },
};

use askama::Template;
use axum::{Extension, extract::Path, http::StatusCode, response::Html};
use tracing::{info, instrument};

#[instrument]
pub async fn list_interviews(Extension(store): Extension<InterviewStore>) -> Html<String> {
    let service = WebService::new(&store.connection);

    let interviews = service.get_interviews().await;

    let interviews_template = InterviewsTemplate { interviews }.render().unwrap();

    info!("queried all interviews");
    Html(interviews_template)
}

#[instrument]
pub async fn htmx_list_interviews(Extension(store): Extension<InterviewStore>) -> Html<String> {
    let service = WebService::new(&store.connection);

    let interviews = service.get_interviews().await;

    let htmx_interviews_template = HtmxInterviewsTemplate { interviews }.render().unwrap();

    info!("queried all interviews");
    Html(htmx_interviews_template)
}
