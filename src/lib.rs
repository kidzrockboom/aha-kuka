use axum::{Router, routing::get};
use simplelog::*;

// Handler that immediately returns an empty `200 OK` response.
async fn health_check() {}

pub async fn app() -> Router {
    // build our application routes
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health_check", get(health_check));
    info!("Created router app");

    app
}
