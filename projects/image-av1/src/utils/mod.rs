use std::io::{Error, ErrorKind};

use image::ImageError;

pub type Result<T> = std::result::Result<T, ImageError>;

pub fn io_error<T, S>(kind: ErrorKind, message: S) -> Result<T>
where
    S: Into<String>,
{
    Err(ImageError::IoError(Error::new(kind, message.into())))
}
