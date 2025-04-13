use actix_web::{HttpResponse, Responder, body::BoxBody, http::header::ContentType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HealthCheckResponse {
    pub(super) status: String,
}

impl HealthCheckResponse {
    pub fn ok() -> Self {
        Self {
            status: "ok".into(),
        }
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
