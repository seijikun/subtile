use super::{PgsDecoder, PgsError};
use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Seek},
    marker::PhantomData,
    path::Path,
};

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
    Reader: BufRead + Seek,
    Decoder: PgsDecoder,
{
    /// create a parser of from a buffered reader (impl [`std::io::BufRead`] trait).
    pub const fn new(reader: Reader) -> Self {
        Self {
            reader,
            phantom_data: PhantomData,
        }
    }

    /// Create a parser for a `*.sup` file from the path of the file.
    #[profiling::function]
    pub fn from_file<P>(path: P) -> Result<SupParser<BufReader<File>, Decoder>, PgsError>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let sup_file = fs::File::open(path).map_err(|source| PgsError::Io {
            source,
            path: path.into(),
        })?;

        let reader = BufReader::new(sup_file);
        Ok(SupParser::new(reader))
    }
}

impl<Reader, Decoder> Iterator for SupParser<Reader, Decoder>
where
    Reader: BufRead + Seek,
    Decoder: PgsDecoder,
{
    type Item = Result<Decoder::Output, PgsError>;

    fn next(&mut self) -> Option<Self::Item> {
        Decoder::parse_next(&mut self.reader).transpose()
    }

    // Set lower bound to promote the allocation of a minimum number of elements.
    fn size_hint(&self) -> (usize, Option<usize>) {
        (500, None)
    }
}
