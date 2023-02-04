use super::controller_type::{HErr, RT};
use axum::{http::StatusCode, Json};
use serde::Serialize;
use serde_json::{json, Value};

#[derive(serde::Serialize)]
struct Response<T: Serialize = ()> {
    code: u16,
    message: String,
    data: Option<T>,
}

pub async fn not_found_handler() -> HErr {
    (
        StatusCode::NOT_FOUND,
        json!(Response::<()> {
            code: 404,
            message: "Not Found".to_string(),
            data: None
        })
        .into(),
    )
}

pub fn accept() -> RT {
    Ok(json!(Response::<()> {
        code: 200,
        message: "OK".to_string(),
        data: None
    })
    .into())
}

pub fn accept_with_message(s: &str) -> RT {
    Ok(json!(Response::<()> {
        code: 200,
        message: s.to_string(),
        data: None
    })
    .into())
}

pub fn accept_with_data(data: impl Serialize) -> RT {
    Ok(json!(Response {
        code: 200,
        message: "OK".to_string(),
        data: Some(data)
    })
    .into())
}

pub fn auth_required() -> HErr {
    (
        StatusCode::BAD_REQUEST,
        json!(Response::<()> {
            code: 400,
            message: "Login Required".to_string(),
            data: None
        })
        .into(),
    )
}

pub fn access_failed() -> HErr {
    (
        StatusCode::BAD_REQUEST,
        json!(Response::<()> {
            code: 400,
            message: "Access Failed".to_string(),
            data: None
        })
        .into(),
    )
}

pub fn err_type_with_code(code: StatusCode, s: &str) -> HErr {
    (
        code,
        json!(Response::<()> {
            code: code.as_u16(),
            message: s.to_string(),
            data: None
        })
        .into(),
    )
}

pub fn internal_error<T>(err: T) -> HErr
where
    T: Into<anyhow::Error>,
{
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        json!(Response::<()> {
            code: 500,
            message: err.into().to_string(),
            data: None
        })
        .into(),
    )
}

pub fn format_error(err: anyhow::Error) -> HErr {
    (
        StatusCode::BAD_REQUEST,
        json!(Response::<()> {
            code: 400,
            message: err.to_string(),
            data: None
        })
        .into(),
    )
}
