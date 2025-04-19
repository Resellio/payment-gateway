use std::fmt::Display;

use actix_web::{
    HttpResponse, ResponseError,
    body::BoxBody,
    http::{StatusCode, header::ContentType},
};
use serde::{Deserialize, Serialize};

pub type AppResult<T> = Result<T, ErrorResponse>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub(crate) error: String,
    #[serde(skip)]
    pub(crate) code: StatusCode,
}

impl ErrorResponse {
    pub fn new(error: String, code: StatusCode) -> Self {
        Self { error, code }
    }
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error: {}", self.error)
    }
}

impl ResponseError for ErrorResponse {
    fn status_code(&self) -> StatusCode {
        self.code
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::build(self.status_code())
            .content_type(ContentType::json())
            .body(body)
    }
}
