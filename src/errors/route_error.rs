use crate::errors::app_error::AppError;
use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::{Request, Response};
use serde::Serialize;
use std::fmt;
use std::fmt::Formatter;
use std::io::Cursor;

pub struct RouteError {
    status: Status,
    description: String,
    source: Option<Box<dyn AppError>>,
}

impl RouteError {
    #[allow(dead_code)]
    pub fn new(status: Status, description: &str) -> Self {
        RouteError {
            status,
            description: description.to_string(),
            source: None,
        }
    }

    pub fn source<S>(status: Status, description: &str, source: S) -> Self
    where
        S: Into<Box<dyn AppError>>,
    {
        RouteError {
            status,
            description: description.to_string(),
            source: Some(source.into()),
        }
    }

    pub fn bad_request(description: &str) -> Self {
        RouteError {
            status: Status::BadRequest,
            description: description.to_string(),
            source: None,
        }
    }
}

impl AppError for RouteError {
    fn source(&self) -> Option<&(dyn AppError)> {
        // See: https://stackoverflow.com/a/65659930/12347616
        if let Some(source) = &self.source {
            Some(&**source)
        } else {
            None
        }
    }

    fn description(&self) -> String {
        self.description.to_string()
    }

    fn get_error_msg(&self) -> String {
        format!("RouteError-{:?}: {:?}", self.status, self.description())
    }
}

impl fmt::Display for RouteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_stacktrace())
    }
}

impl fmt::Debug for RouteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_stacktrace())
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    message: String,
}

impl<'r> Responder<'r, 'static> for RouteError {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        let err_response = serde_json::to_string(&ErrorResponse {
            message: self.description,
        })
        .unwrap();

        Response::build()
            .status(self.status)
            .header(ContentType::JSON)
            .sized_body(err_response.len(), Cursor::new(err_response))
            .ok()
    }
}
