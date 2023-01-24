use crate::{
    controller::{
        response::{accept, accept_with_message},
        RT,
    },
    model::user::{RegisterRequest, User},
    repository::user::{create_user, get_user_by_name, update_user_uuid},
    utility::errors::{internal_error, internal_error_sede},
};
use axum::{extract::State, Extension, Json};
use serde::Deserialize;
use serde_json::{from_value, Value};
use sqlx::PgPool;
use tower_cookies::{Cookie, Cookies};

use super::response::{failed_with_message, user_not_found};

#[axum_macros::debug_handler]
pub async fn register(State(pool): State<PgPool>, Json(user): Json<Value>) -> RT {
    create_user(&pool, serde_json::from_value(user).map_err(internal_error)?)
        .await
        .map_err(internal_error)?;
    accept()
}

#[derive(serde::Deserialize)]
pub struct LoginRequest {
    pub name: String,
    pub password: String,
}
#[axum_macros::debug_handler]
pub async fn login(State(pool): State<PgPool>, cookies: Cookies, Json(user): Json<Value>) -> RT {
    let user: LoginRequest = from_value(user).map_err(internal_error)?;
    let user_db = get_user_by_name(&pool, user.name.as_str())
        .await
        .map_err(internal_error)?
        .ok_or_else(user_not_found)?;
    if user_db.password != user.password {
        return Err(failed_with_message("Password Error"));
    }
    let uuid = uuid::Uuid::new_v4().to_string();
    update_user_uuid(&pool, user_db.id, uuid.as_str())
        .await
        .map_err(internal_error)?;
    let mut cookie = Cookie::new("uuid", uuid);
    cookie.set_path("/");
    cookies.add(cookie);
    accept()
}

#[axum_macros::debug_handler]
pub async fn logout(cookies: Cookies) -> RT {
    let mut cookie = Cookie::new("uuid", "");
    cookie.set_path("/");
    cookies.remove(cookie); // TODO: any better way to remove cookie?
    accept()
}

#[axum_macros::debug_handler]
pub async fn homepage(Extension(user): Extension<User>) -> RT {
    Ok(accept_with_message(
        format!("Hello, {}", user.name.as_str()).as_str(),
    ))
}
