use crate::movies::api::handlers::MoviesApiDoc;
use crate::quotes::api::handlers::QuotesApiDoc;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Quentin Tarantino API",
        description = "A comprehensive API for exploring Quentin Tarantino works",
        version = "1.0.0",
        contact(
            name = "API Support",
            email = "support@example.com"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    servers(
        (url = "http://localhost:3000", description = "Development server"),
    ),

    nest(
        (path = "/api/v1", api = MoviesApiDoc),
        (path = "/api/v1", api = QuotesApiDoc),
    ),
)]
pub struct ApiDoc;
