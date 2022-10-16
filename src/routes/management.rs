use crate::entities::datasets::DatasetInfo;
use crate::errors::apperror::AppError;
use crate::errors::routeerror::RouteError;
use crate::errors::serviceerror::ServiceError;
use crate::services::image_classifier::ImageClassifierArc;
use crate::services::manager::ManagerArc;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

#[get("/datasets")]
pub async fn datasets(manager: &State<ManagerArc>) -> Result<Json<Vec<DatasetInfo>>, RouteError> {
    manager.datasets().map(|it| Json(it)).map_err(|e| {
        e.print_stacktrace();
        RouteError::source(
            Status::InternalServerError,
            "Could not fetch datasets, please try again later",
            e,
        )
    })
}
