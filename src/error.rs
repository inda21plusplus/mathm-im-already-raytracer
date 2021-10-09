use std::io::Error as IOError;

#[derive(Debug)]
pub enum Error {
    EncodingError(png::EncodingError),
    IOError(IOError),
}

impl From<png::EncodingError> for Error {
    fn from(err: png::EncodingError) -> Self {
        Self::EncodingError(err)
    }
}

impl From<IOError> for Error {
    fn from(err: IOError) -> Self {
        Self::IOError(err)
    }
}
