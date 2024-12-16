use thiserror::Error;

#[derive(Debug, Error)]
pub enum FinderError {
    #[error("list error: {kind}")]
    ListError { kind: ListErrorKind },
}

#[derive(Debug, Error)]
pub enum ListErrorKind {
    #[error("null handle")]
    NullHandle,
}

pub type Result<T> = std::result::Result<T, FinderError>;
