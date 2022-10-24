use crate::errors::app_error::AppError;
use crate::errors::db_error::DbError;

use rocket::data::N;
use std::fmt;
use std::fmt::{Debug, Formatter};

#[derive(Debug, Copy, Clone)]
pub enum ServiceErrorKind {
    CrudFailed,
    DatasetFailure,
    NoDataset,
    IllegalArgument,
}

pub struct ServiceError {
    pub kind: ServiceErrorKind,
    description: String,
    source: Option<Box<dyn AppError>>,
}

impl ServiceError {
    pub fn crud_failed_src<E>(description: &str, source: E) -> Self
    where
        E: Into<Box<DbError>>,
    {
        ServiceError {
            kind: ServiceErrorKind::CrudFailed,
            description: description.to_string(),
            source: Some(source.into()),
        }
    }

    pub fn dataset_failure(description: &str) -> Self {
        ServiceError {
            kind: ServiceErrorKind::DatasetFailure,
            description: description.to_string(),
            source: None,
        }
    }

    pub fn dataset_failure_src<E>(description: &str, source: E) -> Self
    where
        E: Into<Box<dyn AppError>>,
    {
        ServiceError {
            kind: ServiceErrorKind::DatasetFailure,
            description: description.to_string(),
            source: Some(source.into()),
        }
    }

    pub fn illegal_argument(description: &str) -> Self {
        ServiceError {
            kind: ServiceErrorKind::IllegalArgument,
            description: description.to_string(),
            source: None,
        }
    }

    pub fn illegal_argument_src<E>(description: &str, source: E) -> Self
    where
        E: Into<Box<dyn AppError>>,
    {
        ServiceError {
            kind: ServiceErrorKind::IllegalArgument,
            description: description.to_string(),
            source: Some(source.into()),
        }
    }

    pub fn no_dataset(description: &str) -> Self {
        ServiceError {
            kind: ServiceErrorKind::NoDataset,
            description: description.to_string(),
            source: None,
        }
    }
}

impl AppError for ServiceError {
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
        format!("ServiceError-{:?}: {:?}", self.kind, self.description())
    }
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_stacktrace())
    }
}

impl fmt::Debug for ServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_stacktrace())
    }
}

impl From<ServiceError> for Box<(dyn AppError)> {
    fn from(err: ServiceError) -> Self {
        Box::new(err)
    }
}
