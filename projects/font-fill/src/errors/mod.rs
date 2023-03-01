

pub type FontFillResult<T> = Result<T, FontFillError>;

#[derive(Debug)]
pub enum FontFillError {
    FileError {
        path: String,
        message: String,
    },
    DecodeError {
        message: String,
    }
}

