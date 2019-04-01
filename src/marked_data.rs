use byteorder::{BigEndian, WriteBytesExt};
use combine::{parser::*, *};
use std::io;

/// Parses a standard dynamically-sized chuck from a JPEG file.
///
/// Not all markers in JPEG are followed by data, or even a data size.
/// This only represents those markers that have data.
/// Markers that are not followed by data include ["Start of Image"](exifsd::soi),
/// ["End of Image"](exifsd::eoi), and ["Reset"](exifsd::rst).
///
/// One marker, the ["Start of Segment" marker](exifsd::ScanSegment), is followed
/// by data, but then that data is also followed by entropy-encoded data that
/// is unmarked.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct MarkedData<'a> {
    pub marker: u8,
    pub data: &'a [u8],
}

impl<'a> MarkedData<'a> {
    /// Parses out marked data from a JPEG file with marker parser `marker`.
    ///
    /// ```
    /// use exifsd::*;
    /// use combine::*;
    ///
    /// // 0xFF - Start of marker
    /// // 0x11 - Marker
    /// // 0x00, 0x03 - Length of data including length (so data length 1)
    /// // 0x01 - The data
    /// let input = &[0xFF, 0x11, 0x00, 0x03, 0x01][..];
    /// let result = MarkedData::parser(token(0x11)).parse(input);
    /// let expected = MarkedData { marker: 0x11, data: &[0x01] };
    /// assert_eq!(result, Ok((expected, &[][..])));
    ///
    /// // Length cannot be less than 2 (here it is 1).
    /// MarkedData::parser(any()).parse(&[0xFF, 0x11, 0x00, 0x01][..]).unwrap_err();
    /// ```
    pub fn parser<I: 'a>(
        marker: impl Parser<Input = I, Output = u8> + 'a,
    ) -> impl Parser<Input = I, Output = MarkedData<'a>> + 'a
    where
        I: RangeStream<Item = u8, Range = &'a [u8]>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
    {
        (byte::byte(0xFF), marker, byte::num::be_u16()).then(|(_, marker, size)| {
            if size >= 2 {
                range::take(size as usize - 2)
                    .map(move |data| Self { marker, data })
                    .left()
            } else {
                unexpected_any("16-bit big-endian size less than 2").right()
            }
        })
    }

    /// Writes the binary representation of the `MarkedData` out to a file.
    ///
    /// ```
    /// use exifsd::*;
    /// use combine::*;
    ///
    /// let input = &[0xFF, 0x11, 0x00, 0x03, 0x01][..];
    /// let marked_data = MarkedData::parser(token(0x11)).parse(input).unwrap().0;
    /// let mut written = vec![];
    /// marked_data.write(&mut written).unwrap();
    /// assert_eq!(input, &written[..]);
    /// ```
    pub fn write<W: WriteBytesExt>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_u8(0xFF)?;
        writer.write_u8(self.marker)?;
        // Include the size of the data field in its own size.
        writer.write_u16::<BigEndian>((self.data.len() + 2) as u16)?;
        writer.write_all(self.data)
    }
}
