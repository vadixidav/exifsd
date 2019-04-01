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
            jpeg.write(&mut std::io::stdout().lock())
                .expect("unable to write jpeg to stdout");
        }
        Err(e) => {
            eprintln!("failed to parse exif data:\n{}", e);
        }
    }
}
