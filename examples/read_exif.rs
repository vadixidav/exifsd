use combine::Parser;
use exifsd::*;
use std::path::PathBuf;
use structopt::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    /// Input jpeg
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn main() {
    let Opt { input } = Opt::from_args();
    let input = std::fs::read(input).expect("failed to read file");
    let result = Jpeg::parser().parse(&input[..]);
    match result {
        Ok((jpeg, _)) => {
            for data in &jpeg.data {
                match data {
                    JpegData::MarkedData(md) => {
                        println!(
                            "Found non-image segment marker {:X} with length {}",
                            md.marker,
                            md.data.len()
                        );
                    }
                    JpegData::ScanSegment(ss) => {
                        println!(
                            "Found image segment with specifier length {} and data length {}",
                            ss.specifier.len(),
                            ss.data.len()
                        );
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("failed to parse exif data:\n{}", e);
        }
    }
}
