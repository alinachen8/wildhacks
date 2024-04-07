use axum::{
    routing::get,
    routing::post,
    Router,
    extract::State,
    handler
};

use sqlx::postgres::PgPoolOptions;
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
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}