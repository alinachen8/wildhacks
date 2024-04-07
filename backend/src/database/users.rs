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

pub async fn get_user(id: i64, pool: &PgPool) -> Result<models::User, sqlx::Error> {
    match sqlx::query("SELECT * FROM users WHERE id = $1;")
        .bind(id)
        .fetch_one(pool).await {
            Ok(res) => { 
                return Ok(models::User {
                    id: res.get("id"),
                    name: res.get("name"),
                });
            }
            Err(_) => { return Err(sqlx::Error::RowNotFound); }
    }
}

pub async fn edit_user(user: models::User, pool: &PgPool) -> Result<(), sqlx::Error> {
    match sqlx::query(
        "UPDATE tables
            SET name = $1
            WHERE id = $2")
    .bind(user.name)
    .bind(user.id)
    .execute(pool).await {
        Ok(_) => { return Ok(()); }
        Err(x) => { return Err(x); }
    }
}

pub async fn delete_user(id: i64, pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM users WHERE id = $1;")
        .bind(id)
        .execute(pool)
        .await?;
    return Ok(());
}