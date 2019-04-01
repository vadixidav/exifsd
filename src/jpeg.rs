use crate::*;
use combine::{parser::*, *};

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use combine::stream::state::State;
    use combine::Parser;
    #[test]
    fn parse_canon_40d_jpg() {
        let bytes = &include_bytes!("../exif-samples/jpg/Canon_40D.jpg")[..];
        Jpeg::parser()
            .easy_parse(State::new(bytes))
            .unwrap_or_else(|err| panic!("{}", err.map_range(|r| format!("{:?}", r))));
    }
}
