use crate::errors::apperror::AppError;
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
    pub fn new(status: Status, description: String) -> Self {
        RouteError {
            status,
            description,
            source: None,
        }
    }

    pub fn new_with_source<S>(status: Status, description: String, source: S) -> Self
    where
        S: Into<Box<dyn AppError>>,
    {
        RouteError {
            status,
            description,
            source: Some(source.into()),
        }
    }
}

impl AppError for RouteError {
    fn source(&self) -> Option<&(dyn AppError + 'static)> {
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
