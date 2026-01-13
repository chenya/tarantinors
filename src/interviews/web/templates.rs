use crate::interviews::web::models::InterviewViewModel;
use askama::Template;

#[derive(Template)]
#[template(path = "pages/interviews.html")]
pub struct InterviewsTemplate {
    pub interviews: Vec<InterviewViewModel>,
}

#[derive(Template)]
#[template(path = "pages/htmx/interviews.html")]
pub struct HtmxInterviewsTemplate {
    pub interviews: Vec<InterviewViewModel>,
}
