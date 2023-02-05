use std::net::SocketAddr;

use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use serde_json::{from_str, json, Value};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", post(handle_post_request));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_post_request(body: String) -> impl IntoResponse {
    let parsed_body = from_str::<Value>(&body);
    if parsed_body.is_err() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"message": "bad body"})),
        );
    }

    return (StatusCode::OK, Json(parsed_body.unwrap()));
}
