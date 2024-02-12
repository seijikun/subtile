//! Read functionalities for Presentation Graphic Stream (.sup)
//!
//! Presentation Graphic Stream (SUP files) `BluRay` Subtitle Format doc :
//! <https://blog.thescorpius.com/index.php/2017/07/15/presentation-graphic-stream-sup-files-bluray-subtitle-format/>
//!
mod decoder;
mod segment;
mod sup;

pub use decoder::PgsDecoder;
pub use sup::SupParser;

use std::{io, path::PathBuf};
use thiserror::Error;

/// Error for `Pgs` handling.
#[derive(Debug, Error)]
pub enum PgsError {
    /// Io error on a path.
    #[error("Io error on '{path}'")]
    Io {
        /// Source error
        source: io::Error,
        /// Path of the file we tried to read
        path: PathBuf,
    },

    /// Invalid segment type code value.
    #[error("Invalid value '{value:#02x}' for Segment Type Code ")]
    SegmentInvalidTypeCode {
        /// Value tried to be Interpréted in Segment Type.
        value: u8,
    },
}
