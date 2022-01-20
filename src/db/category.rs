use crate::{db, models, schema};
use diesel::prelude::*;
use schema::categories::dsl::*;

pub fn find_all() -> Vec<models::category::Category> {
    let con = db::connect_db();
    categories
        .limit(10)
        .load::<models::category::Category>(&con)
        .expect("Error loading categories")
}
