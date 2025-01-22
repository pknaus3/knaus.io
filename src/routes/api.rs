use actix_web::web;
use crate::domain::skill::controller::get_skills;
use crate::domain::experience::controller::get_experiences;
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
       web::scope("/api")
           .service(get_skills)
           .service(get_experiences)
    );
}

