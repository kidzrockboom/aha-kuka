use axum::{
    Router,
    routing::{get, post},
};
use simplelog::*;

// Handler that immediately returns an empty `200 OK` response.
async fn health_check() {}

// Handler that immediately returns an empty `200 OK` response.
async fn subscribe() {}

pub async fn app() -> Router {
    // build our application routes
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe));
    info!("Created router app");

    app
}
