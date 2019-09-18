use snafu::{Snafu, ResultExt, Backtrace, ErrorCompat, ensure};
use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("{}", source))]
    ValidationError {
        source: crate::value_objects::ValidationError,
    },

    /// Represents a resource that has not been found.
    #[snafu(display("resource '{}' was not found", resource))]
    ResourceNotFound {
        resource: String,
    },

    /// NotAuthorized conveys that the caller is not authorized to commit the action.
    #[snafu(display("not authorized"))]
    NotAuthorized,

    /// RepoFailure conveys to the caller that some kind of error happened in a repository.
    /// This might have been a concurrency error, or failure to communicate with the database.
    #[snafu(display("{}", source))]
    RepoFailure {
        source: Box<dyn std::error::Error>,
    },
}

impl From<crate::value_objects::ValidationError> for Error {
    fn from(err: crate::value_objects::ValidationError) -> Self {
        Error::ValidationError {
            source: err,
        }
    }
}
