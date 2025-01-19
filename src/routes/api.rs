use actix_web::{get, web, HttpResponse};
use crate::models::skill::{load_skills, Skill};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
       web::scope("/api")
           .service(get_skills)
    );
}


#[get("/skills")]
pub async fn get_skills() -> actix_web::Result<HttpResponse> {
    let skills: Vec<Skill> = load_skills();

    Ok(HttpResponse::Ok().json(skills))
}
