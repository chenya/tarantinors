pub mod api;
pub mod data;
pub mod web;

use axum::{
    Extension, Router,
    routing::{delete, get, post},
};
use data::store::InterviewStore;

pub fn rest_api_router(interview_store: InterviewStore) -> Router {
    let router = Router::new()
        .route("/interviews", get(api::handlers::get_interviews))
        .route("/interviews", post(api::handlers::add_new_interview))
        .route("/interviews/{quote_id}", get(api::handlers::get_interview))
        .route(
            "/interviews/{quote_id}",
            delete(api::handlers::remove_interview),
        )
        // .fallback(api::handlers::fallback_handler)
        .layer(Extension(interview_store));

    router
}

pub fn web_router(interview_store: InterviewStore) -> Router {
    let router = Router::new()
        .route("/", get(web::handlers::list_interviews))
        // .route("/{quote_id}", get(web::handlers::movie_details))
        .layer(Extension(interview_store));

    router
}

pub fn htmx_web_router(interview_store: InterviewStore) -> Router {
    let router = Router::new()
        .route("/", get(web::handlers::htmx_list_interviews))
        // .route("/{quote_id}", get(web::handlers::movie_details))
        .layer(Extension(interview_store));

    router
}
