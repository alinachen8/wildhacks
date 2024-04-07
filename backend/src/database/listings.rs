use sqlx::{PgPool, Row, Error};
use crate::models::{self, Listing};

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

pub async fn get_listing(id: models::ListingId, pool: &PgPool) -> Result<models::Listing, sqlx::Error> {
    match sqlx::query("SELECT * FROM listings WHERE id = $id;")
        .bind(id)
        .fetch_one(pool).await {
            Ok(res) => { 
                return Ok(Listing {
                    id: res.get("id"),
                    name: res.get("name"), 
                    goal: res.get("goal"),
                    interest: res.get("interest"),
                    image_url: res.get("image_url")
                });
            }
            Err(_) => { return Err(sqlx::Error::RowNotFound); }
        }
}

pub async fn get_list(){}