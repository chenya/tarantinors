use crate::quotes::web::models::QuoteViewModel;
use askama::Template;

#[derive(Template)]
#[template(path = "pages/quotes.html")]
pub struct QuotesTemplate {
    pub quotes: Vec<QuoteViewModel>,
}

#[derive(Template)]
#[template(path = "pages/htmx/quotes.html")]
pub struct HtmxQuotesTemplate {
    pub quotes: Vec<QuoteViewModel>,
}
