

pub type FontFillResult<T> = std::result::Result<T, FontFillError>;

pub enum FontFillError {
    FileNotFound {
        path: String,
    },
}
