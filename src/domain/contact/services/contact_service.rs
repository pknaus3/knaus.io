use actix_web::web::Json;
use crate::domain::contact::schema::contacts::dsl::contacts;
use crate::domain::contact::models::contact::{Contact, NewContact};
use diesel::prelude::*;

pub fn insert_contact(conn: &mut PgConnection, data: &Json<NewContact>) -> Contact {
    use crate::domain::contact::schema::contacts::dsl::*;

    diesel::insert_into(contacts)
        .values(&data.0)
        .get_result(conn)
        .expect("Error creating contact")
}