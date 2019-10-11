use std::convert::From;
use std::io::Error as IOError;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum SlidesError {
    Args(&'static str),
    IO(IOError),
    Utf8(FromUtf8Error),
}

impl From<IOError> for SlidesError {
    fn from(err: IOError) -> Self {
        SlidesError::IO(err)
    }
}

impl From<FromUtf8Error> for SlidesError {
    fn from(err: FromUtf8Error) -> Self {
        SlidesError::Utf8(err)
    }
}
