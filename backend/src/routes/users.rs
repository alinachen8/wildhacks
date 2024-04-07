use serde::{Deserialize, Serialize};
use axum::{extract, response, http::StatusCode};

use crate::AppState;
use crate::State;
use crate::models;

use crate::database::users;

#[derive(Clone, Deserialize)]
pub struct PostUserRequest {
    name: String
}

#[derive(Clone, Serialize)]
pub struct PostUserResponse {
    id: i64
}

#[derive(Clone, Deserialize)]
pub struct GetUserRequest {
    id: i64 //realistically this should be in the parameters. im not doing for the sake of time. 
}

#[derive(Clone, Serialize)]
pub struct GetUserResponse {
    id: i64,
    name: String
}

#[derive(Clone, Deserialize)]
pub struct PatchUserRequest {
    id: i64,
    name: String
}

#[derive(Clone, Serialize)]
pub struct PatchUserResponse {
    id: i64,
    name: String
}

#[derive(Clone, Serialize)]
pub struct DeleteUserRequest {
    id: i64,
}


pub async fn post(State(state): State<AppState>, 
            extract::Json(user): extract::Json<PostUserRequest>) -> Result<response::Json<PostUserResponse>, StatusCode> {
    match users::create_user(user.name, &state.pool).await {
        Ok(x) => { return Ok(response::Json(PostUserResponse{ id: x }))}
        Err(_) => { return Err(StatusCode::INTERNAL_SERVER_ERROR); }
    };
}

pub async fn get(State(state): State<AppState>, 
        extract::Json(request): extract::Json<GetUserRequest>) -> Result<response::Json<GetUserResponse>, StatusCode> {
    match users::get_user(request.id, &state.pool).await {
        Ok(x) => { return Ok(response::Json(GetUserResponse{id: x.id, name: x.name})); }
        Err(_) => { return Err(StatusCode::INTERNAL_SERVER_ERROR); }
    }
}

pub async fn patch(State(state): State<AppState>,
        extract::Json(request): extract::Json<PatchUserRequest>) -> Result<response::Json<PatchUserResponse>, StatusCode> {
    match users::edit_user(models::User{id: request.id, name: request.name.clone()}, &state.pool).await {
        Ok(_) => { return Ok(response::Json(PatchUserResponse{id: request.id, name: request.name.clone()})); }
        Err(_) => { return Err(StatusCode::INTERNAL_SERVER_ERROR); }
    }
}

pub async fn delete(State(state): State<AppState>,
        extract::Json(request): extract::Json<DeleteUserRequest>) -> Result<(), StatusCode> {
    match users::delete_user(request.id, &state.pool).await {
        Ok(_) => { return Ok(()); }
        Err(_) => { return Err(StatusCode::INTERNAL_SERVER_ERROR); }
    }
}