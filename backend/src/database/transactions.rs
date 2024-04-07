use sqlx::{PgPool, Row};

use crate::models;


pub async fn sell_bond(user_id: i64, listing_id: i64, quantity: f64, pool: &PgPool) -> Result<(), sqlx::Error>{
    match sqlx::query("SELECT * FROM portfolio WHERE user_id = $1 AND portfolio = $2;")
        .bind(user_id)
        .bind(listing_id)
        .fetch_one(pool).await {
            Ok(row) => { 
                // user already owns this listing.
                let new_quantity: f64 = row.get::<f64, _>("quantity") - quantity; //THIS SHOULD BE UNDER A LOCK!
                
                match sqlx::query("UPDATE portfolio SET quantity = $1 WHERE user_id = $2 AND portfolio = $3;")
                    .bind(user_id)
                    .bind(listing_id)
                    .bind(new_quantity)
                    .execute(pool)
                    .await {
                        Ok(_) => { return Ok(()); }
                        Err(x) => { return Err(x); }
                    }
            }
            Err(sqlx::Error::RowNotFound) => { 
                // user must buy this listing.
                match sqlx::query("INSERT INTO portfolio (user_id, listing_id, quantity) VALUES ($1, $2, $3);")
                    .bind(user_id)
                    .bind(listing_id)
                    .bind(quantity)
                    .execute(pool)
                    .await {
                        Ok(_) => { return Ok(()); }
                        Err(x) => { return Err(x); }
                    }
             }
            Err(x) => { 
                // something weird happened.
                return Err(x); 
            }
        }
}

pub async fn buy_bond(user_id: i64, listing_id: i64, quantity: f64, pool: &PgPool) -> Result<(), sqlx::Error> {
    match sqlx::query("SELECT * FROM portfolio WHERE user_id = $1 AND portfolio = $2;")
        .bind(user_id)
        .bind(listing_id)
        .fetch_one(pool).await {
            Ok(row) => { 
                // user already owns this listing.
                let new_quantity: f64 = row.get::<f64, _>("quantity") + quantity; //THIS SHOULD BE UNDER A LOCK!
                
                match sqlx::query("UPDATE portfolio SET quantity = $1 WHERE user_id = $2 AND portfolio = $3;")
                    .bind(user_id)
                    .bind(listing_id)
                    .bind(new_quantity)
                    .execute(pool)
                    .await {
                        Ok(_) => { return Ok(()); }
                        Err(x) => { return Err(x); }
                    }
            }
            Err(sqlx::Error::RowNotFound) => { 
                // user must buy this listing.
                match sqlx::query("INSERT INTO portfolio (user_id, listing_id, quantity) VALUES ($1, $2, $3);")
                    .bind(user_id)
                    .bind(listing_id)
                    .bind(quantity)
                    .execute(pool)
                    .await {
                        Ok(_) => { return Ok(()); }
                        Err(x) => { return Err(x); }
                    }
             }
            Err(x) => { 
                // something weird happened.
                return Err(x); 
            }
        }
}

// Returns a vector of (ListingID, quantity owned).
pub async fn list_bonds(user_id: i64, pool: &PgPool) -> Result<Vec<(models::ListingId, f64)>, sqlx::Error> {
    match sqlx::query("SELECT * FROM portfolio WHERE user_id = $1;")
        .bind(user_id)
        .fetch_all(pool)
        .await {
        Ok(rows) => {
            let mut res: Vec<(models::ListingId, f64)> = Vec::new();
            for row in rows {
                res.push((row.get("listing_id"), row.get("quantity")));
            }
            return Ok(res);
        }
        Err(x) => {
            return Err(x);
        }
    }
}