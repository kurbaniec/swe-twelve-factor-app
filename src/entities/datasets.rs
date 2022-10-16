use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct DatasetInfo {
    pub id: i32,
    pub in_use: bool,
    pub created_on: chrono::NaiveDateTime,
}
