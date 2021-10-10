use std::io::Error as IOError;

#[derive(Debug)]
pub enum Error {
    IOError(IOError),
}

impl From<IOError> for Error {
    fn from(err: IOError) -> Self {
        Self::IOError(err)
    }
}
