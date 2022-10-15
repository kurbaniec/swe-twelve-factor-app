use crate::entities::picture_upload::PictureUpload;
use crate::errors::routeerror::RouteError;
use crate::services::image_classifier::ImageClassifierArc;
use crate::services::manager::ManagerArc;
use rocket::form::Form;
use rocket::http::Status;
use rocket::State;
use std::thread;
use std::time::Duration;

#[get("/")]
pub async fn index(
    a: &State<ImageClassifierArc>,
    b: &State<ManagerArc>,
) -> Result<&'static str, RouteError> {
    let test = "Some text";
    println!("{}", test);
    // "Hello, world!"
    let test2 = "Some text";
    Err(RouteError::new(Status::Forbidden, "Hey".to_string()))
}

#[post("/isdog", data = "<upload>")]
pub async fn is_dog(upload: Form<PictureUpload<'_>>) -> Result<&'static str, RouteError> {
    let test = &upload.picture;
    println!("{:?}", test);
    Ok("File received")
}
