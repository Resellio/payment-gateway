use actix_web::{Scope, web};

use super::services::health_check;

pub fn health_scope() -> Scope {
    web::scope("/health").service(health_check)
}
