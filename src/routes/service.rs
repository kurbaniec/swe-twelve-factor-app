use crate::entities::picture_upload::PictureUpload;
use crate::errors::route_error::RouteError;
use crate::repositories::traits::DatasetRepository;
use crate::services::traits::Classify;
use crate::states::app_state::{
    ImageClassifierPtr, ImageClassifierState, ManagerPtr, ManagerState,
};
use rocket::form::Form;
use rocket::http::Status;
use rocket::State;

#[get("/")]
pub async fn index(a: &ImageClassifierState, b: &ManagerState) -> Result<&'static str, RouteError> {
    let test = "Some text";
    println!("{}", test);
    // "Hello, world!"
    let test2 = "Some text";
    Err(RouteError::new(Status::Forbidden, "Hey"))
}

#[post("/isdog", data = "<upload>")]
pub async fn is_dog(upload: Form<PictureUpload<'_>>) -> Result<&'static str, RouteError> {
    let test = &upload.picture;
    println!("{:?}", test);
    Ok("File received")
}
