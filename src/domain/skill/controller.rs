use actix_web::{get, HttpResponse};
use crate::domain::skill::model::{Skill, load_skills};

#[get("/skills")]
pub async fn get_skills() -> actix_web::Result<HttpResponse> {
    let skills: Vec<Skill> = load_skills();

    Ok(HttpResponse::Ok().json(skills))
}


