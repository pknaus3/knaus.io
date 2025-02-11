use chrono::{NaiveDateTime};
use diesel::{Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable, serde::Serialize, serde::Deserialize, Debug)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::domain::contact::schema::contacts)]
pub struct Contact {
    id: i32,
    name: String,
    email: String,
    subject: String,
    body: String,
    created_at: NaiveDateTime
}

#[derive(Insertable, serde::Deserialize, Debug)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::domain::contact::schema::contacts)]
pub struct NewContact {
    name: String,
    email: String,
    subject: String,
    body: String,
}