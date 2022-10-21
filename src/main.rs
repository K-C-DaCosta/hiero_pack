use hiero_pack::*;

#[cfg(feature = "console")]
use clap::{App, Arg};

fn main() {
    #[cfg(feature = "console")]
    match run_app() {
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1)
        }
        Ok(_) => std::process::exit(0),
    }
}

#[cfg(feature = "console")]
fn run_app() -> Result<(), String> {
    let clap_matches = parse_std_in();
    let font_path = clap_matches.value_of("font_path").unwrap();
    let page_paths = clap_matches.values_of("page_paths").unwrap();
    let output = clap_matches.value_of("output_opt");

    let font_bytes = std::fs::read(font_path).map_err(Error::from)?;
    let font_text = String::from_utf8(font_bytes).map_err(Error::from)?;

    let page_data: Vec<_> = page_paths
        .map(|page_path| match std::fs::read(page_path) {
            Ok(data) => data,
            Err(err) => {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            }
        })
        .collect();

    let atlas = HieroAtlas::new()
        .with_pages(page_data)
        .with_font_file(font_text)?
        .build();

    if clap_matches.is_present("dump_opt") {
        atlas
            .bitmap_table
            .iter()
            .filter(|(key, _)| !key.is_whitespace())
            .for_each(|(key, val)| {
                println!("key:'{}',val:{}", key, val);
            });
    }

    //serialize Atlus to a binary format
    let atlas_bytes = bincode::serialize(&atlas).unwrap();

    //attempt write binary blob to disk
    let write_result = match output {
        Some(path) => std::fs::write(path, atlas_bytes),
        None => std::fs::write("atlas.bcode", atlas_bytes),
    };
    write_result.map_err(Error::from)?;

    Ok(())
}

#[cfg(feature = "console")]
fn parse_std_in<'a>() -> clap::ArgMatches<'a> {
    App::new("hiero_pack")
        .version("0.1.0")
        .about("packs the files generated from the hiero font tool")
        .author("Khadeem D.")
        .arg(
            Arg::with_name("font_path")
                .short("f")
                .long("font")
                .help("specify the path to your .fnt file")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("page_paths")
                .short("p")
                .long("pages")
                .help("specify page files")
                .multiple(true)
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output_opt")
                .short("o")
                .long("output")
                .help("the name of the output file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("dump_opt")
                .short("d")
                .long("dump")
                .help("dumps the char info to stdout(for debugging purposes)"),
        )
        .get_matches()
}
