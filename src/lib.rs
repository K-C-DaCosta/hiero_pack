use parse::{HieroToken, HieroTokenizer};
use serde::{Deserialize, Serialize};
use std::collections::*;

mod parse;
pub mod atlas;


pub use atlas::*; 

#[derive(Debug)]
pub enum Error {
    ReadWriteError(std::io::Error),
    Utf8ConvertError(std::string::FromUtf8Error),
    DeserializeError(bincode::Error),
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
        }
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

impl From<bincode::Error> for Error {
    fn from(err: bincode::Error) -> Self {
        Error::DeserializeError(err)
    }
}


pub struct HieroIncomplete<T> {
    inner: T,
}

impl<T> HieroIncomplete<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}



#[test]
fn parse_test() {
    let font_file = std::fs::read_to_string("./fonts/uroob.fnt").unwrap();
    let page_file = std::fs::read("./fonts/uroob.png").unwrap();
    let _atlas = HieroAtlas::new()
        .with_pages(vec![page_file])
        .with_font_file(font_file)
        .expect("file failed to parse")
        .build();
}
