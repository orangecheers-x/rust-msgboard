use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use serde::de::IntoDeserializer;
use serde_json::json;
use sqlx::PgPool;
use tower_cookies::Cookies;

use crate::{
    model::user::User, repository::user::get_user_by_uuid, utility::errors::internal_error,
};

use super::{response::not_auth, HErr};

pub async fn extract_info<B>(
    State(pool): State<PgPool>,
    cookies: Cookies,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, HErr> {
    match cookies.get("uuid") {
        Some(cookie) => {
            let uuid = cookie.value().to_string();
            request.extensions_mut().insert(
                get_user_by_uuid(&pool, uuid.as_str())
                    .await
                    .map_err(internal_error)?,
            );
        }
        None => {
            request.extensions_mut().insert(None::<User>);
        }
    }
    let response = next.run(request).await;
    Ok(response)
}

pub async fn login_required<B>(mut request: Request<B>, next: Next<B>) -> Result<Response, HErr> {
    let user = request
        .extensions()
        .get::<Option<User>>()
        .ok_or((
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"message":"gs"}).into(),
        ))?
        .clone()
        .ok_or(not_auth())?;
    request.extensions_mut().insert(user);
    let response = next.run(request).await;
    Ok(response)
}
