use actix_web::web;
use crate::domain::experience::controller::get_experiences;
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
       web::scope("/api")
           .service(get_experiences)
    );
}

