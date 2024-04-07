use axum::http::StatusCode;
use axum::{handler, extract::State};

use crate::database;
use crate::AppState;

pub async fn post(State(state): State<AppState>) -> Result<String, StatusCode> {
    let x = database::listings::create_listing("Soccer Stadium", 30.0, 1.0, "x.com", &state.pool);

    match x.await {
        Ok(z) => { return Ok(z.to_string()); }
        Err(z) => { 
            println!("{}", z);
            return Err(StatusCode::SEE_OTHER); 
        }
    }
}

