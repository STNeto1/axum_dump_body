use std::net::SocketAddr;

use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use serde::Serialize;
use serde_json::{from_str, Value};

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

#[derive(Debug, Serialize)]
struct _Response {
    message: String,
}

async fn handle_post_request(body: String) -> impl IntoResponse {
    let parsed_body = from_str::<Value>(&body);
    if parsed_body.is_err() {
        return (
            StatusCode::BAD_REQUEST,
            Json(_Response {
                message: "bad request".to_owned(),
            }),
        );
    }

    let body_type = match parsed_body.unwrap() {
        Value::Null => "null".to_owned(),
        Value::Bool(b) => b.to_string(),
        Value::String(s) => s,
        Value::Array(_) => "array".to_owned(),
        Value::Object(_) => "object".to_owned(),
        Value::Number(n) => n.to_string(),
    };

    let res = _Response {
        message: format!("body was {}", body_type),
    };

    return (StatusCode::OK, Json(res));
}
