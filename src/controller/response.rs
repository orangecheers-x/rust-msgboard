use super::controller_type::{HErr, RT};
use axum::{http::StatusCode, Json};
use serde_json::{json, Value};

pub async fn not_found_handler() -> HErr {
    (
        StatusCode::NOT_FOUND,
        json!({"message":"前面的区域以后再来探索吧"}).into(),
    )
}

pub fn accept() -> RT {
    Ok(json!({"message":"ok"}).into())
}

pub fn not_auth() -> HErr {
    (
        StatusCode::UNAUTHORIZED,
        json!({"message":"Login Required"}).into(),
    )
}

pub fn user_not_found() -> HErr {
    (StatusCode::OK, json!({"message":"User Not Found"}).into())
}

pub fn failed_with_message(s: &str) -> HErr {
    (StatusCode::OK, json!({ "message": s }).into())
}

pub fn accept_with_message(s: &str) -> Json<Value> {
    json!({ "message": s }).into()
}
