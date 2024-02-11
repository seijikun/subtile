use std::io::BufRead;

/// To parse `Presentation Graphic Stream` content `BluRay` subtitle format (`.sup` file).
pub struct SupParser<Reader>
where
    Reader: BufRead,
{
    reader: Reader,
}

impl<Reader> SupParser<Reader>
where
    Reader: BufRead,
{
    /// create a parser of from a buffered reader (impl [`std::io::BufRead`] trait).
    pub const fn new(reader: Reader) -> Self {
        Self { reader }
    }
}
