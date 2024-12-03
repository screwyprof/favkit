use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("URL conversion failed")]
    UrlConversion,

    #[error("Invalid path")]
    InvalidPath,

    #[error("Core Foundation error")]
    CoreFoundation,
}

pub type Result<T> = std::result::Result<T, Error>;
