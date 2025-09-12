use axum::{
    Form, Router,
    routing::{get, post},
};
use simplelog::*;

#[derive(serde::Deserialize)]
struct UserData {
    email: String,
    user_name: String,
}

// Handler that immediately returns an empty `200 OK` response.
async fn health_check() {}

// Handler that immediately returns an empty `200 OK` response.
async fn subscribe(Form(_user_data): Form<UserData>) {}

pub async fn app() -> Router {
    // build our application routes
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe));
    info!("Created router app");

    app
}
