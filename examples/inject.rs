use combine::Parser;
use exifsd::*;
use std::path::PathBuf;
use structopt::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    /// JPEG to read data from.
    #[structopt(parse(from_os_str))]
    source: PathBuf,
    /// JPEG to write data to.
    #[structopt(parse(from_os_str))]
    dest: PathBuf,
    /// 8-bit marker (follows 0xFF) to inject/replace.
    #[structopt(default_value = "0xE1")]
    marker: u8,
}

fn main() {
    let Opt {
        source,
        dest: dest_path,
        marker,
    } = Opt::from_args();

    let source = std::fs::read(source).expect("failed to read source file");
    let dest = std::fs::read(&dest_path).expect("failed to read destination file");

    let source = Jpeg::parser()
        .parse(&source[..])
        .expect("failed to parse source exif data")
        .0;

    if let Some(md) = source.data.iter().find_map(|jdata| {
        if let JpegData::MarkedData(md) = jdata {
            if md.marker == marker {
                Some(md)
            } else {
                None
            }
        } else {
            None
        }
    }) {
        let mut dest = Jpeg::parser()
            .parse(&dest[..])
            .expect("failed to parse destination exif data")
            .0;

        dest.inject_marked_data(md.clone());
        let mut dest_file =
            std::fs::File::create(&dest_path).expect("couldn't open destination file for writing");
        dest.write(&mut dest_file)
            .expect("failed to write to destination file");
    } else {
        panic!("failed to find data with the specified marker");
    }
}
