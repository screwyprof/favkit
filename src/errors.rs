use std::path::PathBuf;
use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum FinderError {
    InvalidPath { path: PathBuf, source: Option<std::io::Error> },
    UnsupportedTarget(String),
    SystemError(String),
}

impl fmt::Display for FinderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FinderError::InvalidPath { path, source } => {
                write!(f, "Invalid path: {}", path.display())?;
                if let Some(err) = source {
                    write!(f, " ({})", err)?;
                }
                Ok(())
            }
            FinderError::UnsupportedTarget(msg) => write!(f, "Unsupported target: {}", msg),
            FinderError::SystemError(msg) => write!(f, "System error: {}", msg),
        }
    }
}

impl Error for FinderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            FinderError::InvalidPath { source, .. } => source.as_ref().map(|e| e as &(dyn Error + 'static)),
            _ => None,
        }
    }
}

impl FinderError {
    pub fn invalid_path(path: impl Into<PathBuf>) -> Self {
        Self::InvalidPath {
            path: path.into(),
            source: None,
        }
    }
}

pub type Result<T> = std::result::Result<T, FinderError>;
