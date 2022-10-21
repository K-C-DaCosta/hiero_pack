use super::*; 

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


/// This struct is pretty much just a header, for a sub-image inside the page\
/// To get actual pixel data for the bitmap, just look up the page and read the sub-image at top left (x,y) , borrown-right:(x+width,y+height)
#[derive(Serialize, Deserialize, Default, Copy, Clone)]
pub struct HieroBitmapInfo {
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
impl std::fmt::Display for HieroBitmapInfo {
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