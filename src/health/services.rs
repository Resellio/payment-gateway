use actix_web::{Responder, get};

use crate::health::models::HealthCheckResponse;

#[get("/")]
async fn health_check() -> impl Responder {
    HealthCheckResponse::ok()
}
