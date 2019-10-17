// Re-export cgmath for linear algebra
pub use cgmath::*;

// Error & result type
use failure::Fail;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "{}", _0)]
    IoError(std::io::Error),
    #[fail(display = "{}", _0)]
    Custom(String),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
