use thiserror::Error;

#[derive(Error, Debug)]
pub enum SidebarError {
    #[error("Failed to create sidebar list")]
    CreateList,

    #[error("Failed to get items snapshot")]
    Snapshot,

    #[error("Item not found: {0}")]
    ItemNotFound(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),
}

pub type Result<T> = std::result::Result<T, SidebarError>;
