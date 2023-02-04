use crate::model::{
    user::{RegisterRequest, User},
    user_uuid::UserUUID,
    *,
};
use anyhow::Result;
use sqlx::PgPool;

pub async fn create_user(pool: &PgPool, user: RegisterRequest) -> Result<()> {
    sqlx::query!(
        "insert into users (name, email, password) values ($1, $2, $3)",
        user.name,
        user.email,
        user.password
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn list_user(pool: &PgPool) -> Result<Vec<User>> {
    let users = sqlx::query_as!(User, "select * from users order by id")
        .fetch_all(pool)
        .await?;
    Ok(users)
}

pub async fn get_user(pool: &PgPool, id: i32) -> Result<Option<User>> {
    let user = sqlx::query_as!(User, "select * from users where id = $1", id)
        .fetch_optional(pool)
        .await?;
    Ok(user)
}

pub async fn update_user(pool: &PgPool, user: User) -> Result<()> {
    sqlx::query!(
        "update users set name = $1, email = $2, password = $3 where id = $4",
        user.name,
        user.email,
        user.password,
        user.id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_user(pool: &PgPool, id: i32) -> Result<()> {
    sqlx::query!("delete from users where id = $1", id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_user_by_uuid(pool: &PgPool, uuid: &str) -> Result<Option<User>> {
    let user = sqlx::query_as!(
        User,
        "select users.* from users_uuid join users on users.id=users_uuid.user_id where uuid = $1",
        uuid
    )
    .fetch_optional(pool)
    .await?;
    Ok(user)
}

pub async fn get_user_by_name(pool: &PgPool, name: &str) -> Result<Option<User>> {
    let user = sqlx::query_as!(User, "select * from users where name = $1", name)
        .fetch_optional(pool)
        .await?;
    Ok(user)
}

pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<Option<User>> {
    let user = sqlx::query_as!(User, "select * from users where email = $1", email)
        .fetch_optional(pool)
        .await?;
    Ok(user)
}

pub async fn update_user_uuid(pool: &PgPool, user_id: i32, user_uuid: &str) -> Result<()> {
    sqlx::query!(
        "insert into users_uuid (user_id, uuid) values ($1, $2) on conflict (user_id) do update set uuid = $3 where users_uuid.user_id = $4",
        user_id,
        user_uuid,
        user_uuid,
        user_id
    )
    .execute(pool)
    .await?;
    Ok(())
}
