use std::path::Path;

pub type FontFillResult<T> = Result<T, FontFillError>;

#[derive(Debug)]
pub enum FontFillError {
    FileError { path: Url, message: String },
    DecodeError { message: String },
}

impl From<std::io::Error> for FontFillError {
    fn from(value: std::io::Error) -> Self {
        FontFillError::FileError { path: "".to_string(), message: value.to_string() }
    }
}

impl FontFillError {
    pub fn with_path(path: &Path, message: &str) -> Self {
        FontFillError::FileError { path: path.display().to_string(), message: message.to_string() }
    }
}
