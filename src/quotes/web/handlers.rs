use crate::quotes::{
    data::store::QuoteStore,
    web::{
        models::QuoteViewModel,
        service::WebService,
        templates::{HtmxQuotesTemplate, QuotesTemplate},
    },
};

use askama::Template;
use axum::{Extension, extract::Path, http::StatusCode, response::Html};
use tracing::{info, instrument};

#[instrument]
pub async fn list_quotes(Extension(store): Extension<QuoteStore>) -> Html<String> {
    let service = WebService::new(&store.connection);

    let quotes = service.get_quotes().await;

    let quotes_template = QuotesTemplate { quotes }.render().unwrap();

    info!("queried all quotes");
    Html(quotes_template)
}

#[instrument]
pub async fn htmx_list_quotes(Extension(store): Extension<QuoteStore>) -> Html<String> {
    let service = WebService::new(&store.connection);

    let quotes = service.get_quotes().await;

    let htmx_quotes_template = HtmxQuotesTemplate { quotes }.render().unwrap();

    info!("queried all quotes");
    Html(htmx_quotes_template)
}
