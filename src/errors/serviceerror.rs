use crate::errors::apperror::AppError;
use crate::errors::dberror::DbError;
use crate::errors::routeerror::RouteError;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
#[allow(dead_code)]
pub enum ServiceErrorKind {
    CrudFailed,
}

pub struct ServiceError {
    kind: ServiceErrorKind,
    description: String,
    source: Option<Box<dyn AppError>>,
}

impl ServiceError {
    pub fn crud_failed<E>(description: &str, source: E) -> Self
    where
        E: Into<Box<DbError>>,
    {
        ServiceError {
            kind: ServiceErrorKind::CrudFailed,
            description: description.to_string(),
            source: Some(source.into()),
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
        format!("RouteError-{:?}: {:?}", self.kind, self.description())
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
