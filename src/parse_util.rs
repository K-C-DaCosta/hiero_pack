use super::*;

pub fn parse_common(table: &Vec<Vec<&str>>) -> Result<HieroCommon, &'static str> {
    let mut common = HieroCommon::default();

    if table[1].len() < 7 {
        return Err("'common' line lacks columns");
    }

    match table[1][1].split('=').rev().next() {
        Some(line_height) => common.line_height = line_height.parse().expect("parse error"),
        _ => return Err("line_height error"),
    }

    match table[1][2].split('=').rev().next() {
        Some(base) => common.base = base.parse().expect("parse error"),
        _ => return Err("base error"),
    }

    match table[1][3].split('=').rev().next() {
        Some(scale_w) => common.scale_w = scale_w.parse().expect("parse error"),
        _ => return Err("base error"),
    }

    match table[1][4].split('=').rev().next() {
        Some(scale_h) => common.scale_h = scale_h.parse().expect("parse error"),
        _ => return Err("scale error"),
    }

    Ok(common)
}

pub fn parse_info(table: &Vec<Vec<&str>>) -> Result<HieroInfo, &'static str> {
    let mut info = HieroInfo::default();

    if table[0].len() < 12 {
        return Err("'info' line doesn't have right number of columns");
    }

    match table[0][0] {
        "info" => (),
        _ => return Err("info line not found"),
    }

    match table[0][1].split('=').rev().next() {
        Some(face) => info.face = String::from(face),
        None => return Err("parse error"),
    }

    match table[0][2].split('=').rev().next() {
        Some(size) => info.size = size.parse().expect("parse failed"),
        None => return Err("parse err"),
    }

    match table[0][3].split('=').rev().next() {
        Some(bold) => info.bold = bold.parse().expect("parse failed"),
        None => return Err("parse err"),
    }

    match table[0][4].split('=').rev().next() {
        Some(italic) => info.italic = italic.parse().expect("parse failed"),
        None => return Err("parse err"),
    }
    match table[0][5].split('=').rev().next() {
        Some(char_set) => info.char_set = String::from(char_set),
        None => return Err("parse err"),
    }

    match table[0][6].split('=').rev().next() {
        Some(unicode) => info.unicode = unicode.parse().expect("parse failed"),
        None => return Err("parse err"),
    }

    match table[0][7].split('=').rev().next() {
        Some(stretch_h) => info.stretch_h = stretch_h.parse().expect("parse failed"),
        None => return Err("parse err"),
    }

    match table[0][8].split('=').rev().next() {
        Some(smooth) => info.smooth = smooth.parse().expect("parse failed"),
        None => return Err("parse err"),
    }

    match table[0][9].split('=').rev().next() {
        Some(aa) => info.aa = aa.parse().expect("parse failed"),
        None => return Err("parse err"),
    }

    match table[0][10].split('=').rev().next() {
        Some(padding) => {
            info.padding = padding
                .split(',')
                .map(|num_str| num_str.parse().unwrap())
                .collect::<Vec<i32>>()
        }
        None => return Err("parse err"),
    }
    match table[0][11].split('=').rev().next() {
        Some(spacing) => {
            info.padding = spacing
                .split(',')
                .map(|num_str| num_str.parse().unwrap())
                .collect::<Vec<i32>>()
        }
        None => return Err("parse err"),
    }
    Ok(info)
}

pub fn parse_glyphs(table: &Vec<Vec<&str>>) -> Result<HashMap<char, HieroBitmap>, &'static str> {
    let mut glyph_table: HashMap<char, HieroBitmap> = HashMap::new();

    let mut line_iter = table
        .iter()
        .filter(|table| table[0] == "chars" || table[0] == "char");

    match line_iter.next() {
        Some(_chars_line) => {
            while let Some(char_line) = line_iter.next() {
                let char_data: Vec<i32> = char_line[1..]
                    .iter()
                    .map(|&col| {
                        col.split('=')
                            .rev()
                            .next()
                            .unwrap()
                            .parse()
                            .expect("glyph parse failed")
                    })
                    .collect();
                let id: char = std::char::from_u32(char_data[0] as u32).unwrap();
                glyph_table.insert(
                    id,
                    HieroBitmap {
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
        }
        None => return Err("'chars' line missing"),
    }

    Ok(glyph_table)
}

pub fn parse_kerning_table(table: &Vec<Vec<&str>>) -> Result<HashMap<(char, char), i32>, &'static str> {
    let mut kerning_table = HashMap::new();

    let mut line_iter = table
        .iter()
        .filter(|table| table[0] == "kernings" || table[0] == "kerning");

    match line_iter.next() {
        Some(_chars_line) => {
            while let Some(char_line) = line_iter.next() {
                let kerning_data: Vec<i32> = char_line[1..]
                    .iter()
                    .map(|&col| {
                        col.split('=')
                            .rev()
                            .next()
                            .unwrap()
                            .parse()
                            .expect("kerning parse failed")
                    })
                    .collect();
                let first = std::char::from_u32(kerning_data[0] as u32).unwrap();
                let second = std::char::from_u32(kerning_data[0] as u32).unwrap();
                let amount = kerning_data[0];
                kerning_table.insert((first, second), amount);
            }
        }
        None => return Err("'kernings' line missing"),
    }

    Ok(kerning_table)
}
