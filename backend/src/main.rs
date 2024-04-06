use axum::{
    routing::get,
    Router,
    extract::State
};

use sqlx_postgres::{Postgres, PgPool};
use std::env;

pub mod routes;
pub mod models;
pub mod database;

#[tokio::main]
async fn main() {
    #[derive(Clone)]
    struct AppState {
        pool: PgPool
    }

    let state = AppState {
        pool: PgPool::connect(env::var("DB_URL").unwrap().as_str()).await.unwrap()
    };

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}