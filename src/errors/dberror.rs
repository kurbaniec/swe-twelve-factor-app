use crate::errors::apperror::AppError;
use diesel::serialize::IsNull::No;
use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::{Request, Response};
use serde::Serialize;
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::io::Cursor;

#[derive(Debug)]
#[allow(dead_code)]
pub enum DbErrorKind {
    NotFound,
    ReadFailed,
    UpdateFailed,
    DeleteFailed,
    TransactionFailed,
    Connection
}

pub struct DbError {
    kind: DbErrorKind,
    description: String,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl DbError {
    pub fn new(kind: DbErrorKind, description: String) -> Self {
        DbError {
            kind,
            description,
            source: None,
        }
    }

    pub fn source<E>(kind: DbErrorKind, description: &str, source: E) -> Self
    where
        E: Into<Box<dyn Error + Send + Sync>>,
    {
        DbError {
            kind,
            description: description.to_string(),
            source: Some(source.into()),
        }
    }
}

impl AppError for DbError {
    fn source(&self) -> Option<&(dyn AppError + 'static)> {
        None
    }

    fn description(&self) -> String {
        self.description.to_string()
    }

    fn get_error_msg(&self) -> String {
        let mut error_msg = if let Some(source) = &self.source {
            String::from(format!("Error: {:?}", source.to_string()))
        } else {
            String::from("")
        };
        error_msg.push_str(&format!(
            "DbError-{:?}: {:?}",
            self.kind,
            self.description()
        ));
        error_msg
    }
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_stacktrace())
    }
}

impl fmt::Debug for DbError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_stacktrace())
    }
}
