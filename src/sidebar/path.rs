use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Clone)]
pub struct MacOsPath {
    path: PathBuf,
}

impl MacOsPath {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    pub fn as_path(&self) -> &Path {
        &self.path
    }
}

impl std::fmt::Display for MacOsPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path.display())
    }
}
