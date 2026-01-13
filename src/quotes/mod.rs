pub mod api;
pub mod data;
pub mod web;

use axum::{
    Extension, Router,
    routing::{delete, get, post},
};
use data::store::QuoteStore;

pub fn rest_api_router(quote_store: QuoteStore) -> Router {
    let router = Router::new()
        .route("/quotes", get(api::handlers::get_quotes))
        .route("/quotes", post(api::handlers::add_new_quote))
        .route("/quotes/{quote_id}", get(api::handlers::get_quote))
        .route("/quotes/{quote_id}", delete(api::handlers::remove_quote))
        // .fallback(api::handlers::fallback_handler)
        .layer(Extension(quote_store));

    router
}

pub fn web_router(quote_store: QuoteStore) -> Router {
    let router = Router::new()
        .route("/", get(web::handlers::list_quotes))
        // .route("/{quote_id}", get(web::handlers::movie_details))
        .layer(Extension(quote_store));

    router
}

pub fn htmx_web_router(quote_store: QuoteStore) -> Router {
    let router = Router::new()
        .route("/", get(web::handlers::htmx_list_quotes))
        .layer(Extension(quote_store));

    router
}
