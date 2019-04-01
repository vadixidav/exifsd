use combine::{parser::*, *};

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
    pub fn parser<I: 'a>(
        marker: impl Parser<Input = I, Output = u8> + 'a,
    ) -> impl Parser<Input = I, Output = MarkedData<'a>> + 'a
    where
        I: RangeStream<Item = u8, Range = &'a [u8]>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
    {
        (byte::byte(0xFF), marker, byte::num::be_u16()).then(|(_, marker, size)| {
            range::take(size as usize - 2).map(move |data| Self { marker, data })
        })
    }

    pub fn parse_next<I: 'a>() -> impl Parser<Input = I, Output = MarkedData<'a>> + 'a
    where
        I: RangeStream<Item = u8, Range = &'a [u8]>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
    {
        unimplemented!();
        value(MarkedData {
            marker: 0,
            data: &[],
        })
    }
}
