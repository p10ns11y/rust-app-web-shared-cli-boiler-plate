use axum::{routing::get, Router};
use shared::network::NetworkConfig;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let network_config = NetworkConfig::new("0.0.0.0", 3000);

    // Build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    println!(
        "Web Server starting on {}",
        network_config.connection_string()
    );

    println!("Session ID: {}", Uuid::new_v4().to_string());

    // Run it with hyper on localhost:3000
    let listener = tokio::net::TcpListener::bind(network_config.connection_string())
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
