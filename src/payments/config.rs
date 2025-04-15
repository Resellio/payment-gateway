use actix_web::web;

use super::handlers::process_payment;

pub fn payments_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/payments").service(process_payment));
}
