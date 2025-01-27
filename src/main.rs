use actix_web::{App, HttpServer};
mod routes;
mod domain;
use domain::skill::routes::api::configure as skill_config;
use domain::experience::routes::api::configure as experience_config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(routes::assets::configure)
            .configure(skill_config)
            .configure(experience_config)
            .configure(routes::web::configure)
        }
    )
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
