use actix_web::web;
use crate::domain::skill::controllers::skills::get_skills;
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/skill/api")
            .service(get_skills)
    );
}
