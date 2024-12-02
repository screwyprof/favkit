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

    #[error("Invalid path: {0}")]
    InvalidPath(PathBuf),

    #[error("Failed to create list: {0}")]
    CreateList(String),

    #[error("Failed to get snapshot: {0}")]
    Snapshot(String),

    #[error("Failed to add item: {0}")]
    AddItem(String),

    #[error("Failed to remove item: {0}")]
    RemoveItem(String),

    #[error("Invalid section: {0}")]
    InvalidSection(String),
}

pub type Result<T> = std::result::Result<T, SidebarError>;

impl SidebarError {
    pub fn invalid_path(path: impl Into<PathBuf>) -> Self {
        Self::InvalidPath(path.into())
    }

    pub fn item_not_found(path: impl Into<PathBuf>) -> Self {
        Self::NotFound(format!("Item not found: {:?}", path.into()))
    }
}
