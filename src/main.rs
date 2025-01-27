use actix_web::{App, HttpServer};
mod routes;
mod domain;
use domain::skill::routes::api::configure as skill_config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().configure(routes::assets::configure).configure(skill_config).configure(routes::api::configure).configure(routes::web::configure)
        }
    )
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
