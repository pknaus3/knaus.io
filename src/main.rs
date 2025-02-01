use actix_web::{App, HttpServer};

mod routes;
mod domain;
mod services;
pub mod schema;

use domain::skill::routes::api::configure as skill_config;
use domain::experience::routes::api::configure as experience_config;
use domain::blog::routes::api::config as blog_config;
use dotenvy::dotenv;
use std::env;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .configure(routes::assets::configure)
            .configure(skill_config)
            .configure(blog_config)
            .configure(experience_config)
            .configure(routes::web::configure)
        }
    )
    .bind((env::var("APP_HOST").expect("Key: APP_HOST is missing in .env"), env::var("APP_PORT").unwrap().parse::<u16>().expect("Ket: APP_PORT is invalid in .env")))?
    .run()
    .await
}
