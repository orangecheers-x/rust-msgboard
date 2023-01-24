use anyhow::Result;
use sqlx::PgPool;

use crate::model::message::{CreateMessageModel, Message};

pub async fn get_message_by_id(pool: &PgPool, id: i32) -> Result<Message> {
    let message = sqlx::query_as!(Message, "select * from messages where id = $1", id)
        .fetch_one(pool)
        .await?;
    Ok(message)
}

pub async fn list_message(pool: &PgPool) -> Result<Vec<Message>> {
    let messages = sqlx::query_as!(Message, "select * from messages order by created_at desc")
        .fetch_all(pool)
        .await?;
    Ok(messages)
}

pub async fn create_message(pool: &PgPool, message: CreateMessageModel) -> Result<()> {
    sqlx::query!(
        "insert into messages (user_id, content) values ($1, $2)",
        message.user_id,
        message.content
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_message(pool: &PgPool, message: Message) -> Result<()> {
    sqlx::query!(
        "update messages set content = $1 where id = $2",
        message.content,
        message.id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_message(pool: &PgPool, id: i32) -> Result<()> {
    sqlx::query!("delete from messages where id = $1", id)
        .execute(pool)
        .await?;
    Ok(())
}
