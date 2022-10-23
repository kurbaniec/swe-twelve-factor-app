use crate::errors::app_error::AppError;
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use zip::result::ZipError;

pub struct StdError {
    source: Box<dyn Error + Send + Sync>,
}

impl StdError {
    pub fn new<E>(source: E) -> Self
    where
        E: Into<Box<dyn Error + Send + Sync>>,
    {
        StdError {
            source: source.into(),
        }
    }
}

impl AppError for StdError {
    fn source(&self) -> Option<&(dyn AppError)> {
        None
    }

    fn description(&self) -> String {
        self.source.to_string()
    }

    fn get_error_msg(&self) -> String {
        format!("Error: {:?}", self.description())
    }
}

impl fmt::Display for StdError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_stacktrace())
    }
}

impl fmt::Debug for StdError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_stacktrace())
    }
}

impl From<Box<dyn Error + Send + Sync>> for StdError {
    fn from(err: Box<dyn Error + Send + Sync>) -> Self {
        StdError { source: err }
    }
}

impl From<r2d2::Error> for StdError {
    fn from(err: r2d2::Error) -> Self {
        StdError {
            source: Box::new(err),
        }
    }
}

impl From<diesel::result::Error> for StdError {
    fn from(err: diesel::result::Error) -> Self {
        StdError {
            source: Box::new(err),
        }
    }
}

impl From<std::io::Error> for StdError {
    fn from(err: std::io::Error) -> Self {
        StdError {
            source: Box::new(err),
        }
    }
}

impl From<ZipError> for StdError {
    fn from(err: ZipError) -> Self {
        StdError {
            source: Box::new(err),
        }
    }
}

impl From<StdError> for Box<dyn AppError> {
    fn from(err: StdError) -> Self {
        Box::new(err)
    }
}
