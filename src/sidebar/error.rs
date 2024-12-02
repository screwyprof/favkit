use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SidebarError {
    #[error("Core Foundation error: {0}")]
    CoreFoundation(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Operation failed: {0}")]
    Operation(String),
}

pub type Result<T> = std::result::Result<T, SidebarError>;

impl SidebarError {
    pub fn invalid_path(path: impl Into<PathBuf>) -> Self {
        Self::InvalidInput(format!("Invalid path: {:?}", path.into()))
    }

    pub fn item_not_found(path: impl Into<PathBuf>) -> Self {
        Self::NotFound(format!("Item not found: {:?}", path.into()))
    }
}
