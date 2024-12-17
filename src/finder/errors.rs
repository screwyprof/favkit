use thiserror::Error;

#[derive(Debug, Error)]
pub enum FinderError {
    #[error("failed to create favorites list: null handle")]
    NullListHandle,
    #[error("failed to create snapshot: null handle")]
    NullSnapshotHandle,
    #[error("failed to resolve URL: null handle")]
    NullUrlHandle,
}

pub type Result<T> = std::result::Result<T, FinderError>;
