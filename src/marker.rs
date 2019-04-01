use combine::{parser::*, *};

/// Parses Start of Image marker.
pub fn soi<'a, I: 'a>() -> impl Parser<Input = I, Output = &'a [u8]> + 'a
where
    I: RangeStream<Item = u8, Range = &'a [u8]>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    range::range(&[0xFF, 0xD8][..]).expected("Start of Image marker")
}

/// Parses End of Image marker.
pub fn eoi<'a, I: 'a>() -> impl Parser<Input = I, Output = &'a [u8]> + 'a
where
    I: RangeStream<Item = u8, Range = &'a [u8]>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    range::range(&[0xFF, 0xD9][..]).expected("End of Image marker")
}

/// Parses out a Reset marker.
pub fn rst<'a, I: 'a>() -> impl Parser<Input = I, Output = &'a [u8]> + 'a
where
    I: RangeStream<Item = u8, Range = &'a [u8]>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    range::recognize((token(0xFF), one_of((0..8).map(|n| 0xD0 | n)))).expected("Reset marker")
}
