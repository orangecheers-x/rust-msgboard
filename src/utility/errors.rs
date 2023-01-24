use axum::http::StatusCode;
use serde_json::json;

use crate::controller::HErr;

pub fn internal_error<T>(err: T) -> HErr
where
    T: Into<anyhow::Error>,
{
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        json!({"message":err.into().to_string()}).into(),
    )
}

pub fn internal_error_sede(err: serde_json::Error) -> HErr {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        json!({"message":err.to_string()}).into(),
    )
}
