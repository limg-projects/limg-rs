
pub type Result<T> = ::core::result::Result<T, Error>;

pub enum Error {
    #[cfg(feature = "std")]
    IoError(std::io::Error)
}

#[cfg(feature = "std")]
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

// #[cfg(feature = "alloc")]
// impl std::error::Error for Error {}