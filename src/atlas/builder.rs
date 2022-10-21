use super::*; 

pub trait AtlasBuilder: Sized {
    type AtlasType;
    fn with_font_file(self, font: String) -> Result<Self, &'static str>;
    fn with_pages(self, pages: Vec<Vec<u8>>) -> Self;
    fn build(self) -> Self::AtlasType;
}

impl AtlasBuilder for HieroIncomplete<HieroAtlas> {
    type AtlasType = HieroAtlas;

    fn with_font_file(mut self, fnt_file: String) -> Result<Self, &'static str> {
        let mut table: Vec<Vec<HieroToken>> = Vec::new();
        for line in fnt_file.lines() {
            table.push(HieroTokenizer::tokenize_line(line));
        }
        self.inner.info = parse::parse_info(&table)?;
        self.inner.common = parse::parse_common(&table)?;
        self.inner.bitmap_table = parse::parse_glyphs(&table)?;
        self.inner.kerning_table = parse::parse_kerning_table(&table)?;
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