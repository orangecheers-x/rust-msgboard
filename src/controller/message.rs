use axum::{extract::State, Extension, Json};
use serde::Deserialize;
use serde_json::{from_value, json, Value};
use sqlx::PgPool;

use crate::{
    model::{
        message::{CreateMessageModel, Message},
        user::User,
    },
    repository::message::{create_message, list_message},
    utility::errors::internal_error,
};

use super::{
    response::{accept, json_with_message},
    RT,
};

pub async fn message_list(State(pool): State<PgPool>) -> RT {
    let msgs = list_message(&pool).await.map_err(internal_error)?;
    Ok(json!(msgs).into())
}

#[derive(Deserialize)]
struct SendMessageRequest {
    pub content: String,
}
pub async fn send_message(
    State(pool): State<PgPool>,
    Extension(user): Extension<User>,
    Json(msg): Json<Value>,
) -> RT {
    let msg: SendMessageRequest = from_value(msg).map_err(internal_error)?;
    create_message(
        &pool,
        CreateMessageModel {
            user_id: user.id,
            content: msg.content,
        },
    )
    .await
    .map_err(internal_error)?;
    accept()
}
