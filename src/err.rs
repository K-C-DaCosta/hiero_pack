#[derive(Debug)]
pub enum Error {
    ReadWriteError(std::io::Error),
    Utf8ConvertError(std::string::FromUtf8Error),
    DeserializeError(bincode::Error),
    CustomStatic(&'static str),
    Custom(String),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::ReadWriteError(err)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Error::Utf8ConvertError(err)
    }
}

impl From<Error> for String {
    fn from(err: Error) -> Self {
        match err {
            Error::ReadWriteError(e) => e.to_string(),
            Error::Utf8ConvertError(e) => e.to_string(),
            Error::DeserializeError(e) => e.to_string(),
            Error::Custom(c) => c,
            Error::CustomStatic(c) => Self::from(c),
        }
    }
}

impl From<bincode::Error> for Error {
    fn from(err: bincode::Error) -> Self {
        Error::DeserializeError(err)
    }
}

#[derive(Debug)]
pub enum PageUnpackError {
    InvalidIndex,
    PageDecodeError(png::DecodingError),
}

impl std::fmt::Display for PageUnpackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidIndex => write!(f, "Invalid Index error"),
            Self::PageDecodeError(err) => write!(f, "{}", err),
        }
    }
}
