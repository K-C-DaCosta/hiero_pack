use super::*;

mod builder;
mod common;
mod info;
mod page;

pub use self::{builder::*, common::*, info::*, page::*};

#[derive(Serialize, Deserialize)]
pub struct HieroAtlas {
    pub info: HieroInfo,
    pub common: HieroCommon,
    pub bitmap_table: HashMap<char, HieroBitmapInfo>,
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

    pub fn deserialize(data: Vec<u8>) -> Result<Self, Error> {
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
