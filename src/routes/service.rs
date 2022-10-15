use crate::entities::picture_upload::PictureUpload;
use crate::errors::routeerror::RouteError;
use rocket::form::Form;
use rocket::http::Status;

#[get("/")]
pub async fn index() -> Result<&'static str, RouteError> {
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
