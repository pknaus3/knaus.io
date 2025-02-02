use crate::domain::blog::models::post::Post;
use diesel::prelude::*;
use crate::domain::blog::schema;

pub fn get_posts(conn: &mut PgConnection) -> Vec<Post> {
    use schema::posts::dsl::*;

    posts
        .limit(5)
        .select(Post::as_select())
        .load(conn)
        .expect("test")
}