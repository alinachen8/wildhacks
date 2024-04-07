use axum::{
    routing::get,
    routing::post,
    routing::delete,
    routing::patch,
    Router,
    extract::State,
};

use sqlx::postgres::PgPool;
use std::env;

pub mod routes;
pub mod models;
pub mod database;

#[derive(Clone)]
pub struct AppState {
    pool: PgPool
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let state = AppState {
        pool: PgPool::connect(env::var("DB_URL").unwrap().as_str()).await.unwrap()
    };

    let app = Router::new()
        .route("/listings", post(routes::listings::post))
        .route("/listings", get(routes::listings::get))
        .route("/listings", delete(routes::listings::delete))
        .route("/listings", patch(routes::listings::patch))
        .route("/users", post(routes::users::post))
        .route("/users", get(routes::users::get))
        .route("/users", patch(routes::users::patch))
        .route("/portfolio/sell", patch(routes::portfolio::buy))
        .route("/portfolio/buy", patch(routes::portfolio::sell))
        .route("portfolio", get(routes::portfolio::get_portfolio))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}