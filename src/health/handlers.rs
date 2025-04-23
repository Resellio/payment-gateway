use actix_web::{Responder, get};

use crate::health::models::HealthCheckResponse;

#[get("/")]
async fn health_check() -> impl Responder {
    HealthCheckResponse::ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{App, test};

    #[actix_web::test]
    async fn test_health_check() {
        let app = test::init_service(App::new().service(health_check)).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp: HealthCheckResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp.status, "ok");
    }
}
