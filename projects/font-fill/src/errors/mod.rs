use std::{
    fmt::{Debug, Formatter},
    path::{Path, PathBuf},
    sync::LazyLock,
};

use image::ImageError;
use rav1e::{EncoderStatus, InvalidConfig};
use url::Url;

pub type FontFillResult<T> = Result<T, FontFillError>;

pub static EMPTY_URL: LazyLock<Url> = LazyLock::new(|| Url::parse("https://example.com").unwrap());

pub enum FontFillError {
    FileError { path: PathBuf, message: String },
    EncodeError { message: String },
    DecodeError { message: String },
    RuntimeError { message: String },
}

impl Debug for FontFillError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FontFillError::FileError { path, message } => {
                f.debug_struct("FileError").field("path", &path.display()).field("message", message).finish()
            }
            FontFillError::EncodeError { message } => f.debug_struct("EncodeError").field("message", message).finish(),
            FontFillError::DecodeError { message } => f.debug_struct("DecodeError").field("message", message).finish(),
            FontFillError::RuntimeError { message } => f.debug_struct("RuntimeError").field("message", message).finish(),
        }
    }
}

impl From<ImageError> for FontFillError {
    fn from(value: ImageError) -> Self {
        match value {
            ImageError::Encoding(e) => FontFillError::EncodeError { message: e.to_string() },
            ImageError::Decoding(e) => FontFillError::DecodeError { message: e.to_string() },
            ImageError::Parameter(e) => FontFillError::RuntimeError { message: e.to_string() },
            ImageError::Limits(e) => FontFillError::RuntimeError { message: e.to_string() },
            ImageError::Unsupported(e) => FontFillError::RuntimeError { message: e.to_string() },
            ImageError::IoError(e) => FontFillError::file_error(e.to_string()),
        }
    }
}

impl From<std::io::Error> for FontFillError {
    fn from(value: std::io::Error) -> Self {
        FontFillError::FileError { path: PathBuf::new(), message: value.to_string() }
    }
}

impl From<serde_json::Error> for FontFillError {
    fn from(value: serde_json::Error) -> Self {
        FontFillError::EncodeError { message: value.to_string() }
    }
}

impl From<InvalidConfig> for FontFillError {
    fn from(value: InvalidConfig) -> Self {
        FontFillError::RuntimeError { message: value.to_string() }
    }
}

impl From<EncoderStatus> for FontFillError {
    fn from(value: EncoderStatus) -> Self {
        FontFillError::EncodeError { message: value.to_string() }
    }
}

impl FontFillError {
    pub fn file_error(message: impl Into<String>) -> Self {
        FontFillError::FileError { path: PathBuf::default(), message: message.into() }
    }

    pub fn with_path(mut self, path: &Path) -> Self {
        self.set_path(path);
        self
    }
    pub fn set_path(&mut self, new: &Path) {
        match self {
            FontFillError::DecodeError { .. } => {}
            FontFillError::EncodeError { .. } => {}
            FontFillError::RuntimeError { .. } => {}
            FontFillError::FileError { path, .. } => {
                *path = new.to_path_buf();
            }
        }
    }
}
