use axum::{Router, http::StatusCode, routing::get};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health", get(StatusCode::OK));

    // run our app with hyper, listening globally on port 3000
    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::info!("listening on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
