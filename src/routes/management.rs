use crate::entities::datasets::{DatasetInfo, DatasetUpload};
use crate::errors::app_error::AppError;
use crate::errors::route_error::RouteError;
use rocket::form::Form;

use crate::repositories::traits::DatasetRepository;

use crate::states::app_state::ManagerState;
use rocket::http::Status;
use rocket::serde::json::Json;

#[get("/datasets")]
pub async fn datasets(manager: &ManagerState) -> Result<Json<Vec<DatasetInfo>>, RouteError> {
    manager.datasets().map(|it| Json(it)).map_err(|e| {
        e.print_stacktrace();
        RouteError::source(
            Status::InternalServerError,
            "Could not fetch datasets, please try again later",
            e,
        )
    })
}

#[post("/dataset", data = "<upload>")]
pub async fn add_dataset(
    upload: Form<DatasetUpload<'_>>,
    manager: &ManagerState,
) -> Result<String, RouteError> {
    println!("{:?}", upload.in_use);

    Ok(String::from("hey"))
}
