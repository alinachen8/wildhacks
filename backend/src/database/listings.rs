use sqlx::{PgPool, Row};
use crate::models::{self, Listing};

pub async fn create_listing(name: &String, goal: f64, interest: f64, image_url: &String, pool: &PgPool) -> Result<models::UserId, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO listings (name, goal, interest, image_url) VALUES ($1, $2, $3, $4) RETURNING id;")
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
    sqlx::query("DELETE FROM listings WHERE id = $1;")
        .bind(id)
        .execute(pool)
        .await?;
    return Ok(());
}

pub async fn get_listing(id: models::ListingId, pool: &PgPool) -> Result<models::Listing, sqlx::Error> {
    match sqlx::query("SELECT * FROM listings WHERE id = $1;")
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

pub async fn edit_listing(listing: models::Listing, pool: &PgPool) -> Result<(), sqlx::Error>{
    match sqlx::query(
        "UPDATE listings
            SET name = $1, goal = $2, interest = $3, image_url = $4 
            WHERE id = $5")
    .bind(listing.name)
    .bind(listing.goal)
    .bind(listing.interest)
    .bind(listing.image_url)
    .bind(listing.id)
    .execute(pool).await {
        Ok(_) => { return Ok(()); }
        Err(x) => { return Err(x); }
    }
}