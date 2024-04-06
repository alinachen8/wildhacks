use sqlx_postgres::{PgPool};

use crate::models;

pub fn create_listing(name: String, description: String, goal: f64, interest: f64, pool: &PgPool) -> Result<models::UserId, &'static str> {
    

    return Err("Something bad happened!");
}