use axum::{http::StatusCode, Json};
use serde_json::Value;

pub type HErr = (StatusCode, Json<Value>);
pub type RT = Result<Json<Value>, HErr>;
