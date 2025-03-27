use axum::{
    extract::Multipart,
    response::Html,
    routing::post,
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tokio::net::TcpListener;

pub async fn start_server() {
    let app = Router::new()
        .route("/upload", post(upload_file))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("File Upload Server running on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}

async fn upload_file(mut multipart: Multipart) -> Result<Html<String>, axum::http::StatusCode> {
    println!("🔹 Upload request received");

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("unknown").to_string();
        let data = field.bytes().await.unwrap();
        println!("Received file: {} ({} bytes)", name, data.len());
    }

    println!(" Upload complete!");
    Ok(Html("File uploaded successfully!".to_string()))
}
