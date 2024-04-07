use sqlx::{PgPool, Row};

use crate::models;

pub async fn create_user(name: String, pool: &PgPool) -> Result<models::UserId, sqlx::Error> {
    match sqlx::query(
        "INSERT INTO users (name) VALUES ($1) RETURNING id;"
    )
    .bind(name)
    .fetch_one(pool).await {
        Ok(x) => { return Ok(x.get("id")); }
        Err(x) => { return Err(x); }
    }
}

pub async fn get_user() {

}

pub async fn edit_user() {

}

pub async fn delete_user() {

}