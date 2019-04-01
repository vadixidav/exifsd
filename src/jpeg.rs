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
}

#[cfg(test)]
mod tests {
    use super::*;
    use combine::stream::state::State;
    use combine::Parser;
    #[test]
    fn parse_canon_40d_jpg() {
        let bytes = &include_bytes!("../exif-samples/jpg/Canon_40D.jpg")[..];
        let jpeg = Jpeg::parser()
            .easy_parse(State::new(bytes))
            .unwrap_or_else(|err| panic!("{}", err.map_range(|r| format!("{:?}", r))))
            .0;

        let mut written = vec![];
        jpeg.write(&mut written).unwrap();
        assert_eq!(bytes, &written[..]);
    }
}
