use actix_web::{post, web};

use crate::{
    common::models::AppResult,
    payments::{
        domain::{self, Payment},
        models::{ProcessPaymentRequest, ProcessPaymentSuccessResponse},
    },
};

#[post("/")]
async fn process_payment(
    request: web::Json<ProcessPaymentRequest>,
) -> AppResult<ProcessPaymentSuccessResponse> {
    domain::check_for_potential_error()?;
    let payment = Payment::try_from(request.0)?;
    let transaction_id = domain::process_payment(&payment)?;
    let result = ProcessPaymentSuccessResponse::success(transaction_id);
    Ok(result)
}
