use actix_web::{http::StatusCode, post, web};

use crate::{
    common::models::{AppResult, ErrorResponse},
    payments::models::{ProcessPaymentRequest, ProcessPaymentSuccessResponse},
};

#[post("/")]
async fn process_payment(
    request: web::Json<ProcessPaymentRequest>,
) -> AppResult<ProcessPaymentSuccessResponse> {
    if request.amount == 2.0 {
        return Err(ErrorResponse::new("test".into(), StatusCode::BAD_REQUEST));
    }
    let result = ProcessPaymentSuccessResponse::success("123".into());
    Ok(result)
}
