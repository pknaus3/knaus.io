use actix_web::web;
use crate::domain::blog::controllers::posts::get_posts;
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/blog/api")
            .service(get_posts)
    );
}