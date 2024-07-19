use actix_web::{get, post, web, HttpResponse, Responder, Result};
use uuid::Uuid;

use crate::api::health::health_model::{HelloPirateRequest, HelloPirateResponse};

#[post("/hello-pirate")]
pub async fn hello_pirate(req_body: web::Json<HelloPirateRequest>) -> Result<impl Responder> {
    let mut response = "Ahoy! ".to_owned();
    response.push_str(&req_body.message);

    let result = HelloPirateResponse {
        id: Uuid::new_v4().to_string(),
        message: response,
    };

    Ok(web::Json(result))
}

#[get("/health")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
