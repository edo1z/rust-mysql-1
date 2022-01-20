use crate::models::user;
use crate::{db, schema};
use diesel::prelude::*;
use schema::users::dsl::*;

pub fn find_all() -> Vec<user::User> {
    let con = db::connect_db();
    users
        .limit(10)
        .load::<user::User>(&con)
        .expect("Error loading users")
}
