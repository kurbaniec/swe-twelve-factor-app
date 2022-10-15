use crate::errors::apperror::AppError;
use crate::errors::routeerror::RouteError;
use rocket::http::Status;

#[get("/")]
pub fn index() -> Result<&'static str, RouteError> {
    let test = "Some text";
    println!("{}", test);
    // "Hello, world!"
    let test2 = "Some text";
    Err(RouteError::new(Status::Forbidden, "Hey".to_string()))
}
