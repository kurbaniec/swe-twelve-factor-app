use crate::repositories::schema::datasets;
use chrono::Utc;
use diesel::prelude::*;
use rocket::fs::TempFile;
use serde::{Deserialize, Serialize};

#[derive(FromForm)]
pub struct DatasetUpload<'r> {
    pub in_use: Option<bool>,
    pub data: TempFile<'r>,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = datasets)]
pub struct DatasetInsert {
    in_use: bool,
    data: Vec<u8>,
    created_on: chrono::NaiveDateTime,
}

impl DatasetInsert {
    pub fn new(in_use: bool, data: Vec<u8>) -> Self {
        DatasetInsert {
            in_use,
            data,
            created_on: Utc::now().naive_utc(),
        }
    }
}

impl<'r> From<DatasetUpload<'r>> for DatasetInsert {
    fn from(upload: DatasetUpload<'r>) -> Self {
        let in_use = upload.in_use.unwrap_or(false);
        let path = upload.data.path().expect("Could not obtain file");
        let data = std::fs::read(path).expect("Could not read file");
        DatasetInsert::new(in_use, data)
    }
}

#[derive(Queryable, Serialize)]
pub struct DatasetInfo {
    pub id: i32,
    pub in_use: bool,
    pub created_on: chrono::NaiveDateTime,
}
