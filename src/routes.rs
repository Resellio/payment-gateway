use actix_web::web;

use crate::{health::config::health_config, payments::config::payments_config};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(health_config).configure(payments_config);
}
