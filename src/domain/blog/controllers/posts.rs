use actix_web::{get, HttpResponse};
use crate::domain::blog::models::post::Post;
use crate::domain::blog::services::post_service;

#[get("/posts")]
pub async fn get_posts() -> actix_web::Result<HttpResponse> {
    let result: Vec<Post> = post_service::get_posts();

    Ok(HttpResponse::Ok().json(result))
}