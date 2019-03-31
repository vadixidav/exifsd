use combine::{parser::*, *};

#[derive(Copy, Clone, Debug)]
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
