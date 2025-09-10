use aha_kuka::app;
use tokio::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // Setup
    let addr = spawn_app().await;

    let client = reqwest::Client::new();

    // Test logic
    let response = client
        .get(&format!("{}/health_check", &addr))
        .send()
        .await
        .expect("Failed to execute request.");

    // Asserts
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> String {
    // Set up app
    let app = app();

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    // Start the server in a background task and tokio spawn will clean task when test instance of
    // tokio finishes running
    tokio::spawn(async move {
        axum::serve(listener, app.await.into_make_service())
            .await
            .unwrap();
    });
    format!("http://{}", addr)
}
