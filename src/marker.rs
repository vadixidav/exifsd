use crate::*;
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
        .expected("Start of Image marker")
}

/// Parses out a Reset marker.
pub fn rst<'a, I: 'a>() -> impl Parser<Input = I, Output = ()> + 'a
where
    I: RangeStream<Item = u8, Range = &'a [u8]>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (token(0xFF), one_of((0..8).map(|n| 0xD0 | n)))
        .map(|_| ())
        .expected("reset marker")
}

/// Parses out a Start of Scan segment, including the following entropy-encoded data.
pub fn sos<'a, I: 'a>() -> impl Parser<Input = I, Output = MarkedData<'a>> + 'a
where
    I: RangeStream<Item = u8, Range = &'a [u8]>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    MarkedData::parser(token(0xDA).expected("Start of Scan marker")).skip(entropy_encoded_data())
}

/// Parses out an entropy-encoded data section, including `0xFF` padding.
///
/// ```
/// use exifsd::*;
/// use combine::*;
///
/// let result = entropy_encoded_data().parse(&[0x01, 0xFF, 0x00, 0x02, 0xFF, 0xFF, 0xD9][..]);
///
/// // Note that the marker `[0xFF, 0xD9]` is not consumed.
/// assert_eq!(result, Ok(((), &[0xFF, 0xD9][..])));
///
/// let result = entropy_encoded_data().parse(&[0x01, 0xFF, 0x00, 0x02, 0xFF, 0xFF, 0x00][..]);
///
/// // Note that the marker `[0xFF, 0x00]` is not consumed because it follows the padding.
/// assert_eq!(result, Ok(((), &[0xFF, 0x00][..])));
pub fn entropy_encoded_data<'a, I: 'a>() -> impl Parser<Input = I, Output = ()> + 'a
where
    I: RangeStream<Item = u8, Range = &'a [u8]>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let marker = |m| token(0xFF).skip(look_ahead(m));
    let escape = marker(token(0x00));
    let padding = marker(token(0xFF));
    let unescaped_data = none_of(std::iter::once(0xFF));
    skip_many(choice((attempt(unescaped_data), attempt(escape)))).skip(attempt(padding))
}
