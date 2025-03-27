
use axum::{
    extract::{Multipart},
    response::Html,
    routing::{get, post},
    Router,
};
use std::{fs::File, io::Write, net::SocketAddr};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World! This is the Rust server." }))
        .route("/upload", post(upload_file));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Handles single and batch file uploads
async fn upload_file(mut multipart: Multipart) -> Html<String> {
    let mut filenames = Vec::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let filename = field.file_name().unwrap_or("unknown").to_string();
        let data = field.bytes().await.unwrap();

        let mut file = File::create(format!("uploads/{}", filename)).unwrap();
        file.write_all(&data).unwrap();
        filenames.push(filename);
    }

    Html(format!("<h2>✅ Files Uploaded: {:?}</h2>", filenames))
}
