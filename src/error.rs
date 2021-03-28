use std::path::PathBuf;

pub use thiserror::Error;

#[derive(Error, Debug)]
pub enum TimError {
    #[error("Path does not exists: {0}")]
    PathDoesNotExits(PathBuf),
}
