use std::ops::Deref;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct MacOsPath {
    path: PathBuf,
}

impl MacOsPath {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }
}

impl Deref for MacOsPath {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl AsRef<Path> for MacOsPath {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}

impl<P: AsRef<Path>> PartialEq<P> for MacOsPath {
    fn eq(&self, other: &P) -> bool {
        self.path == other.as_ref()
    }
}

impl std::fmt::Display for MacOsPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path.display())
    }
}
