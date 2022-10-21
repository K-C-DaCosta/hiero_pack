use super::*; 


#[derive(Serialize, Deserialize, Default)]
pub struct HieroCommon {
    pub line_height: i32,
    pub base: i32,
    pub scale_w: i32,
    pub scale_h: i32,
    pub pages: i32,
    pub packed: i32,
}