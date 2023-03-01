use std::{
    fmt::{Debug, Formatter},
    path::Path,
    sync::LazyLock,
};

use url::Url;

pub type FontFillResult<T> = Result<T, FontFillError>;

pub static EMPTY_URL: LazyLock<Url> = LazyLock::new(|| Url::parse("https://example.com").unwrap());

pub enum FontFillError {
    FileError { path: Url, message: String },
    DecodeError { message: String },
}

impl Debug for FontFillError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FontFillError::FileError { path, message } => {
                f.debug_struct("FileError").field("path", &path.as_str()).field("message", message).finish()
            }
            FontFillError::DecodeError { message } => f.debug_struct("DecodeError").field("message", message).finish(),
        }
    }
}

impl From<std::io::Error> for FontFillError {
    fn from(value: std::io::Error) -> Self {
        FontFillError::FileError { path: EMPTY_URL.clone(), message: value.to_string() }
    }
}

impl FontFillError {
    pub fn file_error(message: impl Into<String>) -> Self {
        FontFillError::FileError { path: EMPTY_URL.clone(), message: message.into() }
    }

    pub fn with_path(mut self, path: &Path) -> Self {
        self.set_path(path);
        self
    }
    pub fn set_path(&mut self, new: &Path) {
        match self {
            FontFillError::FileError { path, .. } => match Url::from_file_path(new) {
                Ok(o) => *path = o,
                Err(_) => {}
            },
            FontFillError::DecodeError { .. } => {}
        }
    }
}
