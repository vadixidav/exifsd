use combine::parser::range::range;
use combine::{ParseError, Parser, RangeStream};

pub struct Exif;

// `impl Parser` can be used to create reusable parsers with zero overhead
pub fn jpeg<'a, I: 'a>() -> impl Parser<Input = I, Output = Exif> + 'a
where
    I: RangeStream<Range = &'a [u8]>,
    // Necessary due to rust-lang/rust#24159
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    range(&[0xFF, 0xD8][..]).map(|_| Exif)
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn parse_canon_40d_jpg() {
        assert!(jpeg()
            .parse(&include_bytes!("../exif-samples/jpg/Canon_40D.jpg")[..])
            .is_ok());
    }
}
