use crate::{db, models, schema};
use diesel::prelude::*;
use schema::users::dsl::*;

pub fn find_all() -> Vec<models::user::User> {
    let con = db::connect_db();
    users
        .limit(10)
        .load::<models::user::User>(&con)
        .expect("Error loading users")
}
