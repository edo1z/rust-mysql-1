use crate::{db, models, schema};
use diesel::prelude::*;
use schema::users::dsl::*;

pub fn find_all() -> Result<Vec<models::user::User>, String> {
    let con = db::connect_db();
    let result = users
        .limit(10)
        .load::<models::user::User>(&con)
        .expect("Error loading users");
    Ok(result)
}
