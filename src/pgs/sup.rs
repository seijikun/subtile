use super::PgsDecoder;
use std::{io::BufRead, marker::PhantomData};

/// To parse `Presentation Graphic Stream` content `BluRay` subtitle format (`.sup` file).
pub struct SupParser<Reader, Decoder>
where
    Reader: BufRead,
    Decoder: PgsDecoder,
{
    reader: Reader,
    phantom_data: PhantomData<Decoder>,
}

impl<Reader, Decoder> SupParser<Reader, Decoder>
where
    Reader: BufRead,
    Decoder: PgsDecoder,
{
    /// create a parser of from a buffered reader (impl [`std::io::BufRead`] trait).
    pub const fn new(reader: Reader) -> Self {
        Self {
            reader,
            phantom_data: PhantomData,
        }
    }
}
