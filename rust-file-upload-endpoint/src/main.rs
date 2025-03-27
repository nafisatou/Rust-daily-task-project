use axum::{
    extract::{Multipart, State},
    response::Json,
    routing::{get, post},
    Router,
};
use serde_json::{json, Value};
use std::{fs, net::SocketAddr, sync::Arc, collections::HashMap};
use tokio::{net::TcpListener, sync::Mutex, task};
use uuid::Uuid;

type TaskStore = Arc<Mutex<HashMap<String, String>>>;


#[tokio::main]
async fn main() {
    let upload_dir = "uploads";
    let task_store: TaskStore = Arc::new(Mutex::new(HashMap::new()));

    // Ensure the uploads directory exists
    if !fs::metadata(upload_dir).is_ok() {
        fs::create_dir(upload_dir).expect("Failed to create uploads directory");
    }

    println!("Uploads directory is ready!");

    // Create the router      
    let app = Router::new()
        .route("/", get(|| async { Json(json!({ "message": "Rust File Upload Server" })) }))
        .route("/upload", post(upload_file))
        .with_state(task_store.clone());

    let addr = SocketAddr::from(([127, 0, 0, 5], 3000));
    let listener = TcpListener::bind(addr).await.expect("Failed to bind port");

    println!(" Server running at http://{}", addr);

    axum::serve(listener, app.into_make_service())
        .await
        .expect("Server failed to start");
}

// Asynchronous File Upload Handling 
   
async fn upload_file(
    State(state): State<TaskStore>,
    mut multipart: Multipart,
) -> Json<Value> {
    if let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap_or("unknown").to_string();
        let data = field.bytes().await.unwrap();

        let file_path = format!("uploads/{}", file_name);
        let task_id = Uuid::new_v4().to_string();
        let task_id_clone = task_id.clone(); // ✅ Clone before using in async task

        {
            let mut store = state.lock().await;
            store.insert(task_id.clone(), "Processing".to_string());
        }

        //  Offload file writing as an async task
        let task_store_clone = state.clone();
        task::spawn(async move {
            fs::write(&file_path, &data).expect("Failed to write file");
            let mut store = task_store_clone.lock().await;
            store.insert(task_id_clone.clone(), "Completed".to_string());
            println!(" Uploaded: {} (Task ID: {})", file_name, task_id_clone);
        });

        //  Now, `task_id` is still accessible
        Json(json!({ "task_id": task_id, "status": "Processing" }))
    } else {
        Json(json!({ "error": "No file uploaded" }))
    }
}
