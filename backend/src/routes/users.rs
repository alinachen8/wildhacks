use serde::{Deserialize, Serialize};
use axum::{extract, response, http::StatusCode};

use crate::AppState;
use crate::State;

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
    name: String
}

#[derive(Clone, Deserialize)]
pub struct PatchUserRequest {
    id: i64
}

#[derive(Clone, Serialize)]
pub struct PatchUserResponse {
    name: String
}

pub async fn post(State(state): State<AppState>, 
                    extract::Json(user): extract::Json<PostUserRequest>) -> Result<response::Json<PostUserResponse>, StatusCode> {
    match users::create_user(user.name, &state.pool).await {
        Ok(x) => { return Ok(response::Json(PostUserResponse{ id: x }))}
        Err(_) => { return Err(StatusCode::INTERNAL_SERVER_ERROR); }
    };
}