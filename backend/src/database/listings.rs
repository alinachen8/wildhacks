use sqlx::{PgPool, Row};

use crate::models;

pub async fn create_listing(name: String, description: String, goal: f64, interest: f64, pool: &PgPool) -> Result<models::UserId, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO listings(name, goal, interest, image_url)
                VALUES ($name, $price, $interest, $image_url);")
        .bind(name)
        .bind(description)
        .bind(goal)
        .bind(interest)
        .fetch_one(pool)
        .await?;

    let listing_id: i32 = res.get("id");
    return Ok(listing_id);
}