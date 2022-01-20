use serde::{Deserialize, Serialize};
#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub msg: Option<String>,
    pub age: Option<u32>,
}
