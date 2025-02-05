use actix_web::{web, App, HttpServer, cookie::Key};

mod routes;
mod domain;
pub mod schema;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

use domain::skill::routes::api::configure as skill_config;
use domain::experience::routes::api::configure as experience_config;
use domain::blog::routes::api::config as blog_config;
use domain::contact::routes::api::configure as contact_config;
use dotenvy::dotenv;
use std::env;
use actix_session::{SessionMiddleware};
use actix_session::storage::CookieSessionStore;
use actix_web::middleware::Logger;
use diesel::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                // create cookie based session middleware
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(false)
                    .build()
            )
            // .wrap(Logger::new("%D %r"))
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::assets::configure)
            .configure(experience_config)
            .configure(skill_config)
            .configure(blog_config)
            .configure(contact_config)
            .configure(routes::web::configure)
        }
    )
    .bind(
        (env::var("APP_HOST").expect("Key: APP_HOST is missing in .env"), env::var("APP_PORT").unwrap().parse::<u16>().expect("Ket: APP_PORT is invalid in .env"))
    )?
    .run()
    .await
}
