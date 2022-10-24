use crate::entities::image::ImageUpload;
use crate::errors::route_error::RouteError;

use crate::states::app_state::{ImageClassifierState, ManagerState};
use rocket::form::Form;
use rocket::http::Status;

#[get("/")]
pub async fn index(
    _a: &ImageClassifierState,
    _b: &ManagerState,
) -> Result<&'static str, RouteError> {
    let test = "Some text";
    println!("{}", test);
    // "Hello, world!"
    let _test2 = "Some text";
    Err(RouteError::new(Status::Forbidden, "Hey"))
}

#[post("/isdog", data = "<upload>")]
pub async fn is_dog(upload: Form<ImageUpload<'_>>) -> Result<&'static str, RouteError> {
    let test = &upload.image;
    println!("{:?}", test);
    Ok("File received")
}
