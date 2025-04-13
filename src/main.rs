mod common;
mod health;

use actix_cors::Cors;
use actix_web::{
    App, HttpServer,
    middleware::{Logger, NormalizePath, TrailingSlash},
};
use common::config::Config;
use env_logger::Env;
use health::scopes::health_scope;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::load_from_env();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&config.api_origin)
            .allowed_methods(vec!["GET"]);
        App::new()
            .wrap(NormalizePath::new(TrailingSlash::Always))
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(health_scope())
    })
    .bind(("127.0.0.1", config.port))?
    .run()
    .await
}
