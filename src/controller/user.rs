use crate::{
    controller::{
        response::{accept, internal_error},
        RT,
    },
    model::user::{RegisterRequest, User},
    repository::user::{create_user, get_user_by_email, get_user_by_name, update_user_uuid},
};
use axum::{extract::State, http::StatusCode, Extension, Json};
use serde::Deserialize;
use serde_json::{from_value, json, Value};
use sqlx::PgPool;
use tower_cookies::{Cookie, Cookies};

use super::response::{accept_with_data, accept_with_message, err_type_with_code};

#[axum_macros::debug_handler]
pub async fn register(State(pool): State<PgPool>, Json(user): Json<Value>) -> RT {
    create_user(&pool, serde_json::from_value(user).map_err(internal_error)?)
        .await
        .map_err(internal_error)?;
    accept()
}

#[derive(serde::Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}
#[axum_macros::debug_handler]
pub async fn login(State(pool): State<PgPool>, cookies: Cookies, Json(user): Json<Value>) -> RT {
    let user: LoginRequest = from_value(user).map_err(internal_error)?;
    let user_db = get_user_by_email(&pool, user.email.as_str())
        .await
        .map_err(internal_error)?
        .ok_or_else(|| err_type_with_code(StatusCode::BAD_REQUEST, "User Not Found!"))?;
    if user_db.password != user.password {
        return Err(err_type_with_code(
            StatusCode::BAD_REQUEST,
            "Password Error",
        ));
    }
    let uuid = uuid::Uuid::new_v4().to_string();
    update_user_uuid(&pool, user_db.id, uuid.as_str())
        .await
        .map_err(internal_error)?;
    let mut cookie = Cookie::new("uuid", uuid);
    cookie.set_path("/");
    cookies.add(cookie);
    accept_with_data(user_db)
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
    accept_with_message(std::format!("Welcome, {}!", user.name).as_str())
}
