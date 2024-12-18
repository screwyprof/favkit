use crate::system::favorites::FavoritesError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FinderError {
    #[error("failed to access Finder favorites: {0}")]
    AccessError(#[from] FavoritesError),
}

pub type Result<T> = std::result::Result<T, FinderError>;
