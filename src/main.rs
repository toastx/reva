mod structs;
mod functions;

use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    
    let client = reqwest::Client::new();
    
    let app = Router::new()
        .route("/api/call/", post(functions::make_call))
        .route("/api/list/", get(functions::list_calls))
        .route("/api/detail/:call_id/", get(functions::call_detail))
        .route("/api/recording-url/:call_id/", get(functions::recording_url))
        .with_state(client);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
