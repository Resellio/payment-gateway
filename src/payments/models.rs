use actix_web::{HttpResponse, Responder, body::BoxBody, http::header::ContentType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProcessPaymentRequest {
    pub(super) amount: f64,
    pub(super) currency: String,
    pub(super) card_number: String,
    pub(super) card_expiry: String,
    pub(super) cvv: String,
    pub(super) force_error: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ProcessPaymentSuccessResponse {
    pub(super) transaction_id: String,
    pub(super) status: String,
}

impl ProcessPaymentSuccessResponse {
    pub fn success(transaction_id: String) -> Self {
        Self {
            transaction_id,
            status: "success".into(),
        }
    }
}

impl Responder for ProcessPaymentSuccessResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
