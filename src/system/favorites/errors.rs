use thiserror::Error;

#[derive(Debug, Error)]
pub enum FavoritesError {
    #[error("failed to create favorites list: null handle")]
    NullListHandle,
    #[error("failed to create snapshot: null handle")]
    NullSnapshotHandle,
    #[error("failed to resolve URL: null handle")]
    NullUrlHandle,
    #[error("failed to get display name: null handle")]
    NullDisplayNameHandle,
}

pub type Result<T> = std::result::Result<T, FavoritesError>;
