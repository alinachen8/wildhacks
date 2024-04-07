use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{ extract::State, response, extract };

use axum::extract::Query;

use serde::Deserialize;

use crate::database;
use crate::AppState;
use crate::models;

#[derive(Deserialize)]
pub struct IdOnly {
    pub id: i64
}

#[derive(Deserialize)]
pub struct CreateListing {
    pub name: String,
    pub goal: f64,
    pub interest: f64,
    pub image_url: String,
}

pub async fn post(State(state): State<AppState>, extract::Json(listing): extract::Json<CreateListing> ) -> Result<String, StatusCode> {
    let x = 
        database::listings::create_listing(&listing.name, listing.goal, listing.interest, &listing.image_url, &state.pool);

    match x.await {
        Ok(z) => { return Ok(z.to_string()); }
        Err(z) => { 
            println!("{}", z);
            return Err(StatusCode::SEE_OTHER); 
        }
    }
}

pub async fn get(State(state): State<AppState>, parameters: Query<IdOnly>) -> Result<Response, StatusCode> {
    match database::listings::get_listing(parameters.id, &state.pool).await {
        Ok(x) => { return Ok(response::Json(x).into_response()); }
        Err(_) => { return Err(StatusCode::NOT_FOUND); }
    }
}

pub async fn delete(State(state): State<AppState>, parameters: Query<IdOnly>) -> Result<Response, StatusCode> {
    match database::listings::delete_listing(parameters.id, &state.pool).await {
        Ok(_) => { return Ok(String::from("Success.").into_response()); }
        Err(_) => { return Err(StatusCode::NOT_FOUND); }
    }
}

pub async fn patch(State(state): State<AppState>, extract::Json(listing): extract::Json<models::Listing>) -> Result<Response, StatusCode> {
    match database::listings::edit_listing(listing, &state.pool).await {
        Ok(_) => { return Ok(String::from("Success").into_response()); }
        Err(_) => { return Err(StatusCode::NOT_FOUND); }
    }
}