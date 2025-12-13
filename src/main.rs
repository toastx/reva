mod structs;
mod functions;

use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    
    dotenv::dotenv().ok();
    let client = reqwest::Client::new();
    
    let app = Router::new()
        .route("/call", get(functions::make_call))
        .route("/list", get(functions::list_calls))
        .route("/detail/{call_id}", get(functions::call_detail))
        .route("/recording-url/{call_id}", get(functions::recording_url))
        .with_state(client)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any)
        );


    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("✅ Server running on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
