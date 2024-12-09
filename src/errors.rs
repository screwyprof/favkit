use thiserror::Error;

#[derive(Debug, Error)]
pub enum FinderError {
}

pub type Result<T> = std::result::Result<T, FinderError>;