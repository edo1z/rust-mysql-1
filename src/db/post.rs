use crate::{db, models, schema};
use diesel::prelude::*;
use schema::posts::dsl::*;

pub fn find_all() -> Vec<models::post::Post> {
    let con = db::connect_db();
    posts
        .limit(10)
        .load::<models::post::Post>(&con)
        .expect("Error loading posts")
}
