use thiserror::Error;

pub type Result<T> = std::result::Result<T, XtaskError>;

#[derive(Error, Debug)]
pub enum XtaskError {
    #[error(transparent)]
    FileReadError(#[from] std::io::Error),

    #[error("{0}")]
    AnyError(String),

    #[error("File error: {0}")]
    FileError(#[from] fs_extra::error::Error),
}

impl From<String> for XtaskError {
    fn from(err: String) -> Self {
        XtaskError::AnyError(err)
    }
}
