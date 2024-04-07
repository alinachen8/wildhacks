use axum::http::StatusCode;
use axum::extract;
use axum::response;
use serde::{Deserialize, Serialize};

use crate::database;
use crate::AppState;

#[derive(Deserialize)]
pub struct GetPortfolioRequest {
    pub id: i64
}

#[derive(Serialize)]
pub struct GetPortfolioResponse {
    pub listings: Vec<(i64, f64)>
}

#[derive(Deserialize)]
pub struct BuyListingRequest {
    pub user_id: i64,
    pub listing_id: i64,
    pub quantity: f64,
}

#[derive(Serialize)]

pub struct BuyListingResponse {

}

#[derive(Deserialize)]
pub struct SellListingRequest {
    pub user_id: i64,
    pub listing_id: i64,
    pub quantity: f64,
}

#[derive(Serialize)]
pub struct SellListingResponse {

}

pub async fn sell (extract::State(state): extract::State<AppState>, 
        extract::Json(request): extract::Json<SellListingRequest>) -> Result<response::Json<SellListingResponse>, StatusCode> {
    match database::transactions::sell_bond(request.user_id, request.listing_id, request.quantity, &state.pool).await {
        Ok(_) => { return Ok(response::Json(SellListingResponse{})); }
        Err(_) => { return Err(StatusCode::INTERNAL_SERVER_ERROR); }
    } 
}

pub async fn buy (extract::State(state): extract::State<AppState>, 
        extract::Json(request): extract::Json<BuyListingRequest>) -> Result<response::Json<BuyListingResponse>, StatusCode> {
    match database::transactions::buy_bond(request.user_id, request.listing_id, request.quantity, &state.pool).await {
        Ok(_) => { return Ok(response::Json(BuyListingResponse{})); }
        Err(_) => { println!("!!!"); return Err(StatusCode::INTERNAL_SERVER_ERROR); }
    } 
}

pub async fn get_portfolio (extract::State(state): extract::State<AppState>, 
        extract::Json(request): extract::Json<GetPortfolioRequest>) -> Result<response::Json<GetPortfolioResponse>, StatusCode> {
    match database::transactions::list_bonds(request.id, &state.pool).await {
        Ok(x) => { 
            return Ok(response::Json(GetPortfolioResponse{listings: x}));
        }
        Err(_) => { return Err(StatusCode::INTERNAL_SERVER_ERROR); }
    } 
}
