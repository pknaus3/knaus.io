use actix_web::{post, web, HttpResponse};
use crate::DbPool;
use crate::domain::contact::models::contact::NewContact;
use crate::domain::contact::services::contact_service;

#[post("/contact")]
pub async fn post_contact(pool: web::Data<DbPool>, contact: web::Json<NewContact>) -> actix_web::Result<HttpResponse> {
    let result = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        contact_service::insert_contact(&mut conn, &contact)
    }).await?;

    Ok(HttpResponse::Ok().json(result))
}