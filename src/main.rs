use aha_kuka::app;

use simplelog::*;
use std::fs::File;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Warn,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("main.log").expect("Failed to create log file"),
        ),
    ])
    .unwrap();

    let app = app();

    // run our app with listening globally on port 3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000)); // write address like this to not make typos
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!("Created tcp listener on port {}", addr);

    axum::serve(listener, app.await.into_make_service())
        .await
        .unwrap();
}
