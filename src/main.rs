use actix_web::{App, HttpServer};
mod routes;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().configure(routes::assets::configure).configure(routes::api::configure).configure(routes::web::configure)
        }
    )
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
