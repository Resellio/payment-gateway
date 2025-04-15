use actix_web::{Responder, post, web};

use crate::payments::models::ProcessPaymentRequest;

#[post("/")]
async fn process_payment(request: web::Json<ProcessPaymentRequest>) -> impl Responder {
    format!("Card number: {}", request.card_number)
}
