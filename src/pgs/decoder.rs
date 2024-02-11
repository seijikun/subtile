use std::io::{BufRead, Seek};

use super::PgsError;

/// Trait of `Presentation Graphic Stream` decoding.
pub trait PgsDecoder {
    /// Type of the Output data for the image.
    type Output;

    /// Parse next subtitle `PGS` and return an `Output` value.
    /// The `Output` depending of the data we want to decode.
    ///
    /// # Errors
    /// Return the error happened during parsing or decoding.
    fn parse_next<R>(reader: &mut R) -> Result<Option<Self::Output>, PgsError>
    where
        R: BufRead + Seek;
}
