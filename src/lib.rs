use serde::{Deserialize, Serialize};
use std::collections::*;

mod parse_util;

pub enum PageUnpackError {
    InvalidIndex,
    PageDecodeError(png::DecodingError),
}

impl std::fmt::Display for PageUnpackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidIndex => write!(f, "Invalid Index error"),
            Self::PageDecodeError(err) => write!(f, "{}", err.to_string()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct HieroAtlas {
    pub info: HieroInfo,
    pub common: HieroCommon,
    pub bitmap_table: HashMap<char, HieroBitmap>,
    pub kerning_table: HashMap<(char, char), i32>,
    pub compressed_pages: Vec<Vec<u8>>,
}

impl HieroAtlas {
    pub fn new() -> HieroIncomplete<Self> {
        HieroIncomplete::new(Self {
            info: HieroInfo::default(),
            common: HieroCommon::default(),
            bitmap_table: HashMap::new(),
            kerning_table: HashMap::new(),
            compressed_pages: Vec::new(),
        })
    }
    
    pub fn deserialize(data: Vec<u8>) -> Result<Self, HieroPackError> {
        Ok(bincode::deserialize::<Self>(&data[..])?)
    }

    /// decodes a compressed page and returns a heap allocated image
    pub fn try_unpack_page(&self, page_index: usize) -> Result<HieroPage, PageUnpackError> {
        let opt = self.compressed_pages.get(page_index).map(|page| {
            let decoder = png::Decoder::new(&page[..]);
            decoder.read_info().map(|(png_info, mut reader)| {
                let mut page_buffer = vec![0; png_info.buffer_size()];
                reader.next_frame(&mut page_buffer).unwrap();
                HieroPage::from(png_info).with_pixels(page_buffer)
            })
        });
        match opt {
            Some(Ok(page)) => Ok(page),
            Some(Err(decode_err)) => Err(PageUnpackError::PageDecodeError(decode_err)),
            None => Err(PageUnpackError::InvalidIndex),
        }
    }
}

impl From<bincode::Error> for HieroPackError {
    fn from(err: bincode::Error) -> Self {
        HieroPackError::DeserializeError(err)
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct HieroInfo {
    pub face: String,
    pub size: i32,
    pub bold: i32,
    pub italic: i32,
    pub char_set: String,
    pub unicode: i32,
    pub stretch_h: i32,
    pub smooth: i32,
    pub aa: i32,
    pub padding: Vec<i32>,
    pub spacing: Vec<i32>,
}
#[derive(Serialize, Deserialize, Default, Copy, Clone)]
pub struct PageInfo {
    pub width: u32,
    pub height: u32,
    pub samples: u32, // RGB(samples = 3) or RGBA(samples =4)
    pub line_size: u32,
}

#[derive(Serialize, Deserialize, Default)]
pub struct HieroPage {
    pixels: Vec<u8>,
    info: PageInfo,
}

impl HieroPage {
    pub fn pixels(&self) -> &Vec<u8> {
        &self.pixels
    }

    pub fn info(&self) -> PageInfo {
        self.info
    }

    fn with_pixels(mut self, data: Vec<u8>) -> Self {
        self.pixels = data;
        self
    }
}
impl From<png::OutputInfo> for HieroPage {
    fn from(info: png::OutputInfo) -> Self {
        Self {
            pixels: Vec::new(),
            info: PageInfo {
                width: info.width,
                height: info.height,
                samples: info.color_type.samples() as u32,
                line_size: info.line_size as u32,
            },
        }
    }
}

impl std::fmt::Display for HieroBitmap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[x:{},y:{},w:{},h:{},xoff:{},yoff:{},xadv:{},pg_num:{},channel:{}]",
            self.x,
            self.y,
            self.width,
            self.height,
            self.xoffset,
            self.yoffset,
            self.xadvance,
            self.page,
            self.channel,
        )?;
        Ok(())
    }
}
/// This struct is pretty much just a header, for a sub-image inside the page\
/// To get actual pixel data for the bitmap, just look up the page and read the sub-image at top left (x,y) , borrown-right:(x+width,y+height)
#[derive(Serialize, Deserialize, Default, Copy, Clone)]
pub struct HieroBitmap {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub xoffset: i32,
    pub yoffset: i32,
    pub xadvance: i32,
    pub page: i32,
    pub channel: i32,
}

#[derive(Serialize, Deserialize, Default)]
pub struct HieroCommon {
    line_height: i32,
    base: i32,
    scale_w: i32,
    scale_h: i32,
    pages: i32,
    packed: i32,
}

pub struct HieroIncomplete<T> {
    inner: T,
}

impl<T> HieroIncomplete<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

pub trait AtlasBuilder: Sized {
    type AtlasType;
    fn with_font_file(self, font: String) -> Result<Self, &'static str>;
    fn with_pages(self, pages: Vec<Vec<u8>>) -> Self;
    fn build(self) -> Self::AtlasType;
}

impl AtlasBuilder for HieroIncomplete<HieroAtlas> {
    type AtlasType = HieroAtlas;

    fn with_font_file(mut self, fnt_file: String) -> Result<Self, &'static str> {
        let mut table: Vec<Vec<&str>> = Vec::new();
        for line in fnt_file.lines() {
            table.push(line.split_whitespace().collect());
        }
        self.inner.info = parse_util::parse_info(&mut table)?;
        self.inner.common = parse_util::parse_common(&mut table)?;
        self.inner.bitmap_table = parse_util::parse_glyphs(&mut table)?;
        self.inner.kerning_table = parse_util::parse_kerning_table(&mut table)?;
        Ok(self)
    }

    fn with_pages(mut self, pages: Vec<Vec<u8>>) -> Self {
        self.inner.compressed_pages = pages;
        self
    }

    fn build(self) -> Self::AtlasType {
        self.inner
    }
}
pub enum HieroPackError {
    ReadWriteError(std::io::Error),
    Utf8ConvertError(std::string::FromUtf8Error),
    DeserializeError(bincode::Error),
}

impl From<std::io::Error> for HieroPackError {
    fn from(err: std::io::Error) -> Self {
        HieroPackError::ReadWriteError(err)
    }
}
impl From<std::string::FromUtf8Error> for HieroPackError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        HieroPackError::Utf8ConvertError(err)
    }
}

impl From<HieroPackError> for String {
    fn from(err: HieroPackError) -> Self {
        match err {
            HieroPackError::ReadWriteError(e) => e.to_string(),
            HieroPackError::Utf8ConvertError(e) => e.to_string(),
            HieroPackError::DeserializeError(e) => e.to_string(),
        }
    }
}
