use actix_web::web::{self};

use super::services::health_check;

pub fn health_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/health").service(health_check));
}
