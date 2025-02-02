use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::domain::blog::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    id: i32,
    title: String,
    body: String,
    published: bool,
}