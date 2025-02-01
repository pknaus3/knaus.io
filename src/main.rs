use actix_web::{App, HttpServer};

mod routes;
mod domain;
mod services;
pub mod schema;

use domain::skill::routes::api::configure as skill_config;
use domain::experience::routes::api::configure as experience_config;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use crate::domain::blog::models::post::Post;
use crate::domain::blog::services::post_service;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let results: Vec<Post> = post_service::get_posts();

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }

    HttpServer::new(|| {
        App::new()
            .configure(routes::assets::configure)
            .configure(skill_config)
            .configure(experience_config)
            .configure(routes::web::configure)
        }
    )
    .bind((env::var("APP_HOST").expect("Key: APP_HOST is missing in .env"), env::var("APP_PORT").unwrap().parse::<u16>().expect("Ket: APP_PORT is invalid in .env")))?
    .run()
    .await
}
