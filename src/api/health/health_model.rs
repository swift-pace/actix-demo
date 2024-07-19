use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct HelloPirateRequest {
    pub message: String,
}

#[derive(Serialize)]
pub struct HelloPirateResponse {
    pub id: String,
    pub message: String,
}
