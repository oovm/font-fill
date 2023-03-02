use std::io::{Error, ErrorKind};

use image::ImageError;

pub fn io_error<T, S>(kind: ErrorKind, message: S) -> Result<T, ImageError>
where
    S: Into<String>,
{
    Err(ImageError::IoError(Error::new(kind, message.into())))
}
