#![feature(once_cell)]

use axum::{
    middleware::{from_fn, from_fn_with_state},
    routing::{get, post},
    Router, Server,
};
use sqlx::postgres::PgPoolOptions;
use std::{
    env,
    net::{IpAddr, SocketAddr},
};
use tower_cookies::CookieManagerLayer;
use tower_http::trace::TraceLayer;
use tracing::{info, instrument};
use tracing_core::Level;

use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};

use backend::controller::{self, middleware::extract_info};
use dotenv::dotenv;

// static DB: SyncOnceCe

#[tokio::main]
#[instrument]
async fn start() -> Result<(), anyhow::Error> {
    tracing_subscriber::registry()
        .with(
            filter::Targets::new()
                .with_target("tower_http::trace", Level::DEBUG)
                .with_target("backend", Level::DEBUG),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    info!("Starting server...");
    info!(
        "Listening on {}:{}\n",
        env::var("LISTEN_IP")?,
        env::var("LISTEN_PORT")?
    );

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL")?)
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

    let addr = SocketAddr::new(
        IpAddr::V4(env::var("LISTEN_IP")?.parse()?),
        env::var("LISTEN_PORT")?.parse()?,
    );
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
    dotenv().ok();
    start().unwrap()
}
