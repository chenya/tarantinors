use crate::movies::web::models::{ErrorViewModel, MovieViewModel};
use askama::Template;
#[derive(Template)]
#[template(path = "base.html")]
pub struct BaseTemplate;

#[derive(Template)]
#[template(path = "pages/home.html")]
pub struct HomeTemplate;

#[derive(Template)]
#[template(path = "pages/movie_detail.html")]
pub struct MovieDetailsTemplate {
    pub movie: MovieViewModel,
}

#[derive(Template)]
#[template(path = "pages/movies.html")]
pub struct MoviesTemplate {
    pub movies: Vec<MovieViewModel>,
}

#[derive(Template)]
#[template(path = "pages/error.html")]
pub struct ErrorTemplate {
    pub error: ErrorViewModel,
}
