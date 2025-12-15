use askama::Template;

#[derive(Template)]
#[template(path = "pages/movies.html")]
pub struct MoviesTemplate {
    pub movies: Vec<Movie>,
}

#[derive(Template)]
#[template(path = "pages/movie_detail.html")]
pub struct MovieDetailTemplate {
    pub movie: Movie,
}
