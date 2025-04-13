mod common;
mod health;

use actix_web::{
    App, HttpServer,
    middleware::{NormalizePath, TrailingSlash},
};
use common::config::Config;
use health::scopes::health_scope;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::load_from_env();
    HttpServer::new(|| {
        App::new()
            .wrap(NormalizePath::new(TrailingSlash::Always))
            .service(health_scope())
    })
    .bind(("127.0.0.1", config.port))?
    .run()
    .await
}
