use actix_web::web;
use crate::domain::experience::controllers::experiences::get_experiences;
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/experience/api")
            .service(get_experiences)
    );
}

