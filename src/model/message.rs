use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct Message {
    pub id: i32,
    pub user_id: i32,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct CreateMessageModel {
    pub user_id: i32,
    pub content: String,
}
