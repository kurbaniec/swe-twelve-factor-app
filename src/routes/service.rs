use crate::entities::image::{ImageClassification, ImageUpload};
use crate::errors::app_error::AppError;
use crate::errors::route_error::RouteError;
use crate::errors::service_error::ServiceErrorKind;
use crate::states::app_state::{ImageClassifierState, ManagerState};
use rocket::form::Form;
use rocket::http::Status;
use rocket::serde::json::Json;

#[post("/dogorcat", data = "<upload>")]
pub async fn dog_or_cat(
    upload: Form<ImageUpload<'_>>,
    ic: &ImageClassifierState,
) -> Result<Json<ImageClassification>, RouteError> {
    ic.classify_image(upload.into_inner())
        .map(Json)
        .map_err(|e| {
            e.print_stacktrace();
            match e.kind {
                ServiceErrorKind::NoDataset => RouteError::source(
                    Status::BadRequest,
                    "No dataset loaded, please load one and try again later",
                    e,
                ),
                ServiceErrorKind::IllegalArgument => {
                    RouteError::source(Status::BadRequest, &e.description(), e)
                }
                _ => RouteError::source(
                    Status::InternalServerError,
                    "Could not classify image, please try again later",
                    e,
                ),
            }
        })
}
