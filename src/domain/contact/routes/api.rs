use actix_web::web;
use crate::domain::contact::controllers::contacts::post_contact;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/contact/api")
            .service(post_contact)
    );
}