use std::{path::PathBuf};

pub use thiserror::Error;

#[derive(Error, Debug)]
pub enum TimError {
    #[error("Path does not exists: {0}")]
    PathDoesNotExits(PathBuf),
    #[error("Build system can not be found.")]
    BuildSystemCannotBeFound,
    #[error("Error occured while building tests.")]
    BuildSystemError,
    #[error("Unexpected error is occured. Message: {0}")]
    UnexpectedError(anyhow::Error),
}
