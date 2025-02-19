use crate::domain::experience::models::experience::{load_experience, Experience};
use actix_web::{get, HttpResponse};

#[get("/experiences")]
pub async fn get_experiences() -> actix_web::Result<HttpResponse> {
    let experiences: Vec<Experience> = load_experience();

    Ok(HttpResponse::Ok().json(experiences))
}
