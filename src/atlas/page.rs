use super::*; 

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

    pub fn with_pixels(mut self, data: Vec<u8>) -> Self {
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