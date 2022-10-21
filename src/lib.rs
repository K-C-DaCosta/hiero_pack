use parse::{HieroToken, HieroTokenizer};
use serde::{Deserialize, Serialize};
use std::collections::*;

mod parse;
mod err;
pub mod atlas;

pub use self::{atlas::*,err::*}; 

pub struct Incomplete<T> {
    inner: T,
}

impl<T> Incomplete<T> {
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
