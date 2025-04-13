mod health;

use actix_web::{
    App, HttpServer,
    middleware::{NormalizePath, TrailingSlash},
};
use health::scopes::health_scope;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(NormalizePath::new(TrailingSlash::Always))
            .service(health_scope())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
