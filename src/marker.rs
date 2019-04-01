use combine::{parser::*, *};

/// Parses Start of Image marker.
pub fn soi<'a, I: 'a>() -> impl Parser<Input = I, Output = ()> + 'a
where
    I: RangeStream<Item = u8, Range = &'a [u8]>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    range::range(&[0xFF, 0xD8][..])
        .map(|_| ())
        .expected("Start of Image marker")
}

/// Parses End of Image marker.
pub fn eoi<'a, I: 'a>() -> impl Parser<Input = I, Output = ()> + 'a
where
    I: RangeStream<Item = u8, Range = &'a [u8]>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    range::range(&[0xFF, 0xD9][..])
        .map(|_| ())
        .expected("End of Image marker")
}
