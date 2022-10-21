use super::*;

mod tests;

#[derive(Clone, Copy, Debug)]
pub struct InSlice {
    lbound: usize,
    bytes: usize,
}

impl InSlice {
    pub fn new(lbound: usize) -> Self {
        Self { lbound, bytes: 0 }
    }

    pub fn push(&mut self, character: char) {
        self.bytes += character.len_utf8();
    }

    pub fn compute_slice<'a>(&self, text: &'a str) -> &'a str {
        &text[self.lbound..self.lbound + self.bytes]
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HieroToken<'a> {
    EntryName(&'a str),
    Pair { key: &'a str, val: &'a str },
}

impl<'a> HieroToken<'a> {
    pub fn as_entry(&self) -> Option<&'a str> {
        if let Self::EntryName(entry) = self {
            Some(entry)
        } else {
            None
        }
    }

    pub fn as_pair(&'a self) -> Option<(&'a str, &'a str)> {
        if let &Self::Pair { key, val } = self {
            Some((key, val))
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum HieroTokenizer {
    Start,
    Header,
    KeyWhite,
    Key,
    Value,
    ValueQuote,
    ValueWhite,
}
impl HieroTokenizer {
    pub fn tokenize_line(line: &str) -> Vec<HieroToken<'_>> {
        let mut byte_cursor = 0;
        let mut res = vec![];
        let mut state = HieroTokenizer::Start;
        let mut accum = InSlice::new(byte_cursor);
        let mut _header = "";
        let mut key = "";
        let mut _val = "";

        for c in line.chars().chain([' ']) {
            match state {
                Self::Start => {
                    if c.is_alphabetic() {
                        accum = InSlice::new(byte_cursor);
                        accum.push(c);
                        state = Self::Header;
                    }
                }
                Self::Header => {
                    if c.is_whitespace() {
                        _header = accum.compute_slice(line);
                        res.push(HieroToken::EntryName(_header));
                        state = Self::KeyWhite;
                    } else {
                        accum.push(c);
                    }
                }
                Self::KeyWhite => {
                    if !c.is_whitespace() {
                        accum = InSlice::new(byte_cursor);
                        accum.push(c);
                        state = Self::Key;
                    }
                }
                Self::Key => {
                    if c == '=' {
                        key = accum.compute_slice(line);
                        accum = InSlice::new(byte_cursor + c.len_utf8());
                        state = Self::Value;
                    } else {
                        accum.push(c);
                    }
                }
                Self::Value => {
                    if c == '\"' {
                        accum = InSlice::new(byte_cursor + c.len_utf8());
                        state = Self::ValueQuote;
                    } else if c.is_whitespace() {
                        _val = accum.compute_slice(line);
                        res.push(HieroToken::Pair { key, val: _val });

                        //clear pointers to make it easier to follow in debugger
                        // key = "";
                        // val = "";
                        state = Self::ValueWhite;
                    } else {
                        accum.push(c);
                    }
                }
                Self::ValueQuote => {
                    if c == '"' {
                        _val = accum.compute_slice(line);
                        res.push(HieroToken::Pair { key, val: _val });

                        //clear pointers to make it easier to follow in debugger
                        // key = "";
                        // val = "";
                        state = Self::ValueWhite;
                    } else {
                        accum.push(c);
                    }
                }
                Self::ValueWhite => {
                    if !c.is_whitespace() {
                        accum = InSlice::new(byte_cursor);
                        accum.push(c);
                        state = Self::Key;
                    }
                }
            }
            byte_cursor += c.len_utf8();
        }
        res
    }
}

pub fn parse_common(table: &[Vec<HieroToken>]) -> Result<HieroCommon, &'static str> {
    let mut common = HieroCommon::default();
    let common_line = &table[1];

    if common_line.len() < 7 {
        return Err("'common' line lacks columns");
    }

    common.line_height = find_pair_by_key(common_line, "lineHeight")
        .ok_or("lineHeight missing")?
        .1
        .parse()
        .ok()
        .ok_or("cant parse lineHeight")?;

    common.base = find_pair_by_key(common_line, "base")
        .ok_or("base missing")?
        .1
        .parse()
        .ok()
        .ok_or("cant parse: base")?;

    common.scale_w = find_pair_by_key(common_line, "scaleW")
        .ok_or("scaleW missing")?
        .1
        .parse()
        .ok()
        .ok_or("cant parse: scaleW")?;

    common.scale_h = find_pair_by_key(common_line, "scaleH")
        .ok_or("scaleH missing")?
        .1
        .parse()
        .ok()
        .ok_or("cant parse: scaleH")?;

    common.pages = find_pair_by_key(common_line, "pages")
        .ok_or("pages missing")?
        .1
        .parse()
        .ok()
        .ok_or("cant parse: pages")?;

    common.packed = find_pair_by_key(common_line, "packed")
        .ok_or("packed missing")?
        .1
        .parse()
        .ok()
        .ok_or("cant packed lineHeight")?;

    Ok(common)
}

pub fn parse_info(table: &[Vec<HieroToken>]) -> Result<HieroInfo, &'static str> {
    let mut info = HieroInfo::default();

    let info_line = &table[0];
    if info_line.len() < 12 {
        return Err("'info' line doesn't have right number of columns");
    }

    info_line
        .iter()
        .find_map(|tok| tok.as_entry())
        .ok_or("info line not found")?;

    info.face = find_pair_by_key(info_line, "face")
        .ok_or("face not found")?
        .1
        .into();

    info.size = find_pair_by_key(info_line, "size")
        .ok_or("size not found")?
        .1
        .parse()
        .ok()
        .ok_or("size failed to parse")?;

    info.bold = find_pair_by_key(info_line, "bold")
        .ok_or("bold not found")?
        .1
        .parse()
        .ok()
        .ok_or("bold bailed to parse")?;

    info.italic = find_pair_by_key(info_line, "italic")
        .ok_or("italic not found")?
        .1
        .parse()
        .ok()
        .ok_or("italic parse failed")?;

    info.face = find_pair_by_key(info_line, "face")
        .ok_or("face not found")?
        .1
        .into();

    info.char_set = find_pair_by_key(info_line, "charset")
        .ok_or("charset not found")?
        .1
        .into();

    info.unicode = find_pair_by_key(info_line, "unicode")
        .ok_or("charset not found")?
        .1
        .parse()
        .ok()
        .ok_or("unicode failed to parse")?;

    info.stretch_h = find_pair_by_key(info_line, "stretchH")
        .ok_or("charset not found")?
        .1
        .parse()
        .ok()
        .ok_or("unicode failed to parse")?;

    info.smooth = find_pair_by_key(info_line, "smooth")
        .ok_or("smooth not found")?
        .1
        .parse()
        .ok()
        .ok_or("smooth failed to parse")?;

    info.aa = find_pair_by_key(info_line, "aa")
        .ok_or("aa not found")?
        .1
        .parse()
        .ok()
        .ok_or("aa failed to parse")?;

    info.padding = find_pair_by_key(info_line, "padding")
        .ok_or("padding not found")?
        .1
        .split(',')
        .filter_map(|num| num.parse().ok())
        .collect();

    info.spacing = find_pair_by_key(info_line, "spacing")
        .ok_or("spacing not found")?
        .1
        .split(',')
        .filter_map(|num| num.parse().ok())
        .collect();
    Ok(info)
}

pub fn parse_glyphs(table: &[Vec<HieroToken>]) -> Result<HashMap<char, HieroBitmapInfo>, &'static str> {
    let mut glyph_table: HashMap<char, HieroBitmapInfo> = HashMap::new();

    let line_iter = table
        .iter()
        .filter(|table| table[0].as_entry().is_some() && table[0].as_entry().unwrap() == "char");

    for char_line in line_iter {
        let char_data: Vec<i32> = char_line[1..]
            .iter()
            .filter_map(|tok| tok.as_pair())
            .filter_map(|(_key, val)| val.parse().ok())
            .collect();
        let id: char = std::char::from_u32(char_data[0] as u32).unwrap();
        glyph_table.insert(
            id,
            HieroBitmapInfo {
                x: char_data[1],
                y: char_data[2],
                width: char_data[3],
                height: char_data[4],
                xoffset: char_data[5],
                yoffset: char_data[6],
                xadvance: char_data[7],
                page: char_data[8],
                channel: char_data[9],
            },
        );
    }

    Ok(glyph_table)
}

pub fn parse_kerning_table(
    table: &[Vec<HieroToken>],
) -> Result<HashMap<(char, char), i32>, &'static str> {
    let mut kerning_table = HashMap::new();

    let line_iter = table
        .iter()
        .filter(|table| table[0].as_entry().is_some() && table[0].as_entry().unwrap() == "kerning");

    for char_line in line_iter {
        let kerning_data: Vec<i32> = char_line[1..]
            .iter()
            .flat_map(|tok| tok.as_pair())
            .filter_map(|(_, val)| val.parse().ok())
            .collect();
        let first = std::char::from_u32(kerning_data[0] as u32).unwrap();
        let second = std::char::from_u32(kerning_data[0] as u32).unwrap();
        let amount = kerning_data[0];
        kerning_table.insert((first, second), amount);
    }

    Ok(kerning_table)
}

fn find_pair_by_key<'a>(line: &'a [HieroToken], key: &'a str) -> Option<(&'a str, &'a str)> {
    line.iter()
        .filter_map(|tok| tok.as_pair())
        .find(|(k, _v)| k == &key)
}
