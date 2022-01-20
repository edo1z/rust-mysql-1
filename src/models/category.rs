use serde::{Deserialize, Serialize};
#[derive(Queryable, Serialize, Deserialize)]
pub struct Category {
    pub id: u32,
    pub name: String,
}
