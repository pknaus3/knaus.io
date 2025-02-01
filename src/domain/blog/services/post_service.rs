use crate::domain::blog::models::post::Post;
use crate::services::database::establish_connection;
use diesel::prelude::*;
pub fn get_posts() -> Vec<Post> {

    use crate::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let results: Vec<Post> = posts
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    results
}