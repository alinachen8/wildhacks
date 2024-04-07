use sqlx::{PgPool, Row};
use crate::models;

pub async fn create_listing(name: &str, goal: f64, interest: f64, image_url: &str, pool: &PgPool) -> Result<models::UserId, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO listings (name, goal, interest, image_url) VALUES ('$1', $2, $3, '$4') RETURNING id;")
        .bind(name)
        .bind(goal)
        .bind(interest)
        .bind(image_url)
        .fetch_one(pool)
        .await?;
    
    let listing_id: i64 = res.get("id");
    return Ok(listing_id);
}

pub async fn delete_listing(id: models::ListingId, pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM listings WHERE id = $id;")
        .bind(id)
        .execute(pool)
        .await?;

    return Ok(());
}

pub async fn get_list(){}