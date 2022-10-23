use crate::errors::app_error::AppError;





use std::error::Error;
use std::fmt;
use std::fmt::Formatter;


#[derive(Debug)]
#[allow(dead_code)]
pub enum DbErrorKind {
    NotFound,
    ReadFailed,
    UpdateFailed,
    DeleteFailed,
    TransactionFailed,
    Connection,
}

pub struct DbError {
    kind: DbErrorKind,
    description: String,
    source: Option<Box<dyn AppError>>,
}

impl DbError {
    pub fn new(kind: DbErrorKind, description: String) -> Self {
        DbError {
            kind,
            description,
            source: None,
        }
    }

    pub fn source<S>(kind: DbErrorKind, description: &str, source: S) -> Self
    where
        S: Into<Box<dyn AppError>>,
    {
        DbError {
            kind,
            description: description.to_string(),
            source: Some(source.into()),
        }
    }
}

impl AppError for DbError {
    fn source(&self) -> Option<&(dyn AppError)> {
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
        format!("DbError-{:?}: {:?}", self.kind, self.description())
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
