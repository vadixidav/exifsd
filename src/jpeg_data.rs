use crate::*;
use combine::{parser::*, *};

#[derive(Clone, Debug)]
pub enum JpegData<'a> {
    MarkedData(MarkedData<'a>),
    ScanSegment(ScanSegment<'a>),
}

impl<'a> JpegData<'a> {
    /// Parses any data in a JPEG between the Start of Image marker and the End of Image marker.
    pub fn parser<I: 'a>() -> impl Parser<Input = I, Output = JpegData<'a>> + 'a
    where
        I: RangeStream<Item = u8, Range = &'a [u8]>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
    {
        choice((
            attempt(ScanSegment::parser().map(JpegData::ScanSegment)),
            // If it wasn't a Scan Segment, the only other option is End of Image (0xD9).
            attempt(MarkedData::parser(none_of(std::iter::once(0xD9))).map(JpegData::MarkedData)),
        ))
    }
}
