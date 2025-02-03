use actix_web::web::Json;
use crate::domain::contact::schema::contacts::dsl::contacts;
use crate::domain::contact::models::contact::Contact;
use diesel::prelude::*;

pub fn insert_contact(conn: &mut PgConnection, data: &Json<Contact>) -> usize {
    use crate::domain::contact::schema::contacts::dsl::*;


    diesel::insert_into(contacts)
        .values(&data.0)
        .execute(conn)
        .expect("Error creating contact")
}