use crate::*;
use byteorder::{BigEndian, WriteBytesExt};
use combine::{parser::*, *};
use std::io;

#[derive(Clone, Debug)]
pub struct Jpeg<'a> {
    pub data: Vec<JpegData<'a>>,
}

impl<'a> Jpeg<'a> {
    pub fn parser<I: 'a>() -> impl Parser<Input = I, Output = Jpeg<'a>> + 'a
    where
        I: RangeStream<Item = u8, Range = &'a [u8]>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
    {
        soi()
            .with(many(JpegData::parser()))
            .skip(eoi())
            .map(|data| Jpeg { data })
    }

    /// Writes the binary representation of the `Jpeg` out to a file.
    ///
    /// Note that the resulting file is a valid JPEG file.
    pub fn write<W: WriteBytesExt>(&self, writer: &mut W) -> io::Result<()> {
        // Start of Image marker
        writer.write_u16::<BigEndian>(0xFFD8)?;
        for data in &self.data {
            data.write(writer)?;
        }
        // End of Image marker
        writer.write_u16::<BigEndian>(0xFFD9)
    }

    /// Writes or replaces the existing `MarkedData` section with `marked_data`.
    pub fn inject_marked_data(&mut self, marked_data: MarkedData<'a>) {
        for data in &mut self.data {
            if let JpegData::MarkedData(md) = data {
                if md.marker == marked_data.marker {
                    *md = marked_data;
                    return;
                }
            }
        }
        // Insert the data at the beginning otherwise.
        self.data.insert(0, JpegData::MarkedData(marked_data));
    }

    /// Injects a marker from `src` as per [`inject_marked_data`].
    ///
    /// Returns `true` on success.
    pub fn inject_marker_from(&mut self, src: &Jpeg<'a>, marker: u8) -> bool {
        if let Some(md) = src.data.iter().find_map(|jdata| {
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
            self.inject_marked_data(*md);
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use combine::stream::state::State;
    use combine::Parser;
    use walkdir::WalkDir;

    #[test]
    fn parse_and_reencode_every_test_jpeg() {
        for jpeg_file in WalkDir::new("exif-samples/jpg") {
            // If we fail due to permissions, we don't want the test to succeed.
            let jpeg_file = jpeg_file.expect("failed to inspect test file");
            // Skip directories.
            if !jpeg_file
                .path()
                .extension()
                .map(|s| s == "jpg")
                .unwrap_or(false)
            {
                continue;
            }

            eprintln!("Testing {}", jpeg_file.path().display());

            let bytes = std::fs::read(jpeg_file.path()).expect("failed to open test file");

            if jpeg_file.path().file_name().unwrap() == "corrupted.jpg" {
                // The corrupted file should fail.
                Jpeg::parser()
                    .easy_parse(State::new(&bytes[..]))
                    .expect_err("should get error when parsing corrupted.jpg");
            } else {
                // All other files should parse and encode back into themselves.
                let jpeg = Jpeg::parser()
                    .easy_parse(State::new(&bytes[..]))
                    .unwrap_or_else(|err| panic!("{}", err.map_range(|r| format!("{:?}", r))))
                    .0;

                let mut written = vec![];
                jpeg.write(&mut written).unwrap();
                let same = &bytes[0..written.len()] == written.as_slice();
                assert!(same);
            }
        }
    }
}
