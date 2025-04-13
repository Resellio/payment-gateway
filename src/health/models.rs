use actix_web::{HttpResponse, Responder, body::BoxBody, http::header::ContentType};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthCheckResponse {
    status: &'static str,
}

impl HealthCheckResponse {
    pub fn ok() -> Self {
        Self { status: "ok" }
    }
}

impl Responder for HealthCheckResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
