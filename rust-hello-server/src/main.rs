use axum::{
    extract::Multipart,
    response::Html,
    routing::post,
    Router,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::task;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/upload", post(upload_file))
        .layer(CorsLayer::permissive()); // Allow CORS for testing

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("🚀 Server running on http://{}", addr);

    // Use Tokio's listener instead of Hyper's Server
    axum::serve(listener, app).await.unwrap();
}

async fn upload_file(mut _multipart: Multipart) -> Html<String> {
    Html("File upload endpoint".to_string())
}
