use crate::entities::datasets::{DatasetInfo, DatasetUpload};
use crate::errors::app_error::AppError;
use crate::errors::route_error::RouteError;
use rocket::form::Form;
use rocket::fs::NamedFile;
use std::fmt::format;
use std::fs;

use crate::repositories::traits::DatasetRepository;

use crate::errors::std_error::StdError;
use crate::states::app_state::ManagerState;
use rocket::http::{ContentType, Status};
use rocket::serde::json::Json;

#[get("/datasets")]
pub async fn datasets(manager: &ManagerState) -> Result<Json<Vec<DatasetInfo>>, RouteError> {
    manager.datasets().map(Json).map_err(|e| {
        e.print_stacktrace();
        RouteError::source(
            Status::InternalServerError,
            "Could not fetch datasets, please try again later",
            e,
        )
    })
}

#[get("/dataset/<id>")]
pub async fn dataset_data(id: i32, manager: &ManagerState) -> Result<NamedFile, RouteError> {
    let path = manager.dataset_data(id).map_err(|e| {
        e.print_stacktrace();
        RouteError::source(
            Status::InternalServerError,
            "Could not fetch dataset data, please try again later",
            e,
        )
    })?;

    let nfile = NamedFile::open(&path)
        .await
        .map_err(StdError::from)
        .map_err(|e| {
            e.print_stacktrace();
            RouteError::source(
                Status::InternalServerError,
                "Could not return dataset data, please try again later",
                e,
            )
        })?;

    let base_path = path.parent().unwrap();
    let _ = fs::remove_dir_all(&base_path)
        .map_err(StdError::from)
        .map_err(|e| {
            println!("Remove directory {:?} manually", base_path);
            e.print_stacktrace();
        });

    Ok(nfile)
}

#[post("/dataset", data = "<upload>")]
pub async fn add_dataset(
    upload: Form<DatasetUpload<'_>>,
    manager: &ManagerState,
) -> Result<Json<DatasetInfo>, RouteError> {
    // Check if data is a .zip
    if let Some(ct) = upload.data.content_type() {
        if *ct != ContentType::ZIP {
            return Err(RouteError::bad_request(&format!(
                "Datasets must be zipped, .{} is not supported",
                ct.extension()
                    .map(|c| c.to_string())
                    .unwrap_or_else(|| ct.to_string())
            )));
        }
    } else {
        return Err(RouteError::bad_request("Could not determine content type"));
    }

    manager
        .add_dataset(upload.into_inner())
        .map(Json)
        .map_err(|e| {
            e.print_stacktrace();
            RouteError::source(
                Status::InternalServerError,
                "Could not add dataset, please try again later",
                e,
            )
        })
}
