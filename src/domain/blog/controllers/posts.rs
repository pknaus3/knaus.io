use actix_web::{get, web, HttpResponse};
use crate::DbPool;
use crate::domain::blog::services::post_service;

#[get("/posts")]
pub async fn get_posts(pool: web::Data<DbPool>) -> actix_web::Result<HttpResponse> {
    let result = web::block(move || {
        let mut conn = pool.get().expect("Error getting connection from pool");

        post_service::get_posts(&mut conn)
    }).await?;

    Ok(HttpResponse::Ok().json(result))
}