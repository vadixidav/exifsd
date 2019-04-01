use crate::*;
use byteorder::{BigEndian, WriteBytesExt};
use combine::{parser::*, *};
use std::io;

/// A Scan Segment is contains a segment of the JPEG data.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ScanSegment<'a> {
    /// Specifies which segment is encoded following this specifier.
    pub specifier: &'a [u8],
    /// The actual entropy-encoded data that makes up the image segment.
    pub data: &'a [u8],
}

impl<'a> ScanSegment<'a> {
    /// Parses out a Scan Segment, including the entropy-encoded data.
    ///
    /// ```
    /// use exifsd::*;
    /// use combine::*;
    ///
    /// let input = &[0xFF, 0xDA, 0x00, 0x02, 0x01, 0xFF, 0x00, 0x02, 0xFF, 0xFF, 0xD9][..];
    /// let result = ScanSegment::parser().parse(input);
    /// let expected = ScanSegment { specifier: &[], data: &[0x01, 0xFF, 0x00, 0x02, 0xFF] };
    ///
    /// // Note that the marker `[0xFF, 0xD9]` is not consumed.
    /// assert_eq!(result, Ok((expected, &[0xFF, 0xD9][..])));
    /// ```
    pub fn parser<I: 'a>() -> impl Parser<Input = I, Output = ScanSegment<'a>> + 'a
    where
        I: RangeStream<Item = u8, Range = &'a [u8]>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
    {
        MarkedData::parser(token(0xDA).expected("Start of Scan marker")).then(|md| {
            segment_data().map(move |data| ScanSegment {
                specifier: md.data,
                data,
            })
        })
    }

    /// Writes the binary representation of the `ScanSegment` out to a file.
    ///
    /// ```
    /// use exifsd::*;
    /// use combine::*;
    ///
    /// let input = &[0xFF, 0xDA, 0x00, 0x02, 0x01, 0xFF, 0x00, 0x02][..];
    /// let scan_segment = ScanSegment::parser().parse(input).unwrap().0;
    /// let mut written = vec![];
    /// scan_segment.write(&mut written).unwrap();
    /// assert_eq!(input, &written[..]);
    /// ```
    pub fn write<W: WriteBytesExt>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_u8(0xFF)?;
        writer.write_u8(0xDA)?;
        // Include the size of the data field in its own size.
        writer.write_u16::<BigEndian>((self.specifier.len() + 2) as u16)?;
        writer.write_all(self.specifier)?;
        writer.write_all(self.data)
    }
}

/// Parses out an entropy-encoded data section, including `0xFF` padding.
///
/// ```
/// use exifsd::*;
/// use combine::*;
///
/// let result = segment_data().parse(&[0x01, 0xFF, 0x00, 0x02, 0xFF, 0xFF, 0xD9][..]);
///
/// // Note that the marker `[0xFF, 0xD9]` is not consumed.
/// assert_eq!(result, Ok(((&[0x01, 0xFF, 0x00, 0x02, 0xFF][..]), &[0xFF, 0xD9][..])));
///
/// let result = segment_data().parse(&[0x01, 0xFF, 0x00, 0x02, 0xFF, 0xFF, 0x00][..]);
///
/// // Note that the marker `[0xFF, 0x00]` is not consumed because it follows the padding.
/// assert_eq!(result, Ok(((&[0x01, 0xFF, 0x00, 0x02, 0xFF][..]), &[0xFF, 0x00][..])));
/// ```
pub fn segment_data<'a, I: 'a>() -> impl Parser<Input = I, Output = &'a [u8]> + 'a
where
    I: RangeStream<Item = u8, Range = &'a [u8]>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let marker = |m| token(0xFF).skip(look_ahead(m));
    let escape = marker(token(0x00));
    let padding = marker(token(0xFF));
    let unescaped_data = none_of(std::iter::once(0xFF));
    range::recognize(
        skip_many(choice((attempt(unescaped_data), attempt(escape))))
            .skip(skip_many(attempt(padding))),
    )
}
