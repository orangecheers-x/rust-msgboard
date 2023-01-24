#![feature(once_cell)]

use axum::{
    middleware::{from_fn, from_fn_with_state},
    routing::{get, post},
    Router, Server,
};
use sqlx::postgres::PgPoolOptions;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};
use tower_cookies::CookieManagerLayer;
use tower_http::trace::TraceLayer;
use tracing_core::Level;
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};

use backend::controller::{self, middleware::extract_info};

// static DB: SyncOnceCe

#[tokio::main]
async fn start() -> Result<(), anyhow::Error> {
    tracing_subscriber::registry()
        .with(filter::Targets::new().with_target("tower_http::trace", Level::DEBUG))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://msgboard:msgboard@192.168.114.12:4321/msgboard")
        .await?;

    let user_router = Router::new()
        .route("/login", post(controller::user::login))
        .route("/register", post(controller::user::register))
        .nest(
            "/",
            Router::new()
                .route("/homepage", get(controller::user::homepage))
                .route("/logout", get(controller::user::logout))
                .layer(from_fn(controller::middleware::login_required)),
        );

    let message = Router::new()
        .route("/list", get(controller::message::message_list))
        .nest(
            "/",
            Router::new()
                .route("/send", post(controller::message::send_message))
                .layer(from_fn(controller::middleware::login_required)),
        );

    let app = Router::new()
        .fallback(controller::response::not_found_handler)
        .nest("/user", user_router)
        .nest("/message", message)
        .layer(from_fn_with_state(pool.clone(), extract_info))
        .layer(TraceLayer::new_for_http())
        .layer(CookieManagerLayer::new())
        .with_state(pool);

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
    tracing::debug!("lb");
    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
    println!("signal shutdown");
}

fn main() {
    start().unwrap()
}
