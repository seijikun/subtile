//! Read functionalities for Presentation Graphic Stream (.sup)
//!
//! Presentation Graphic Stream (SUP files) `BluRay` Subtitle Format doc :
//! <https://blog.thescorpius.com/index.php/2017/07/15/presentation-graphic-stream-sup-files-bluray-subtitle-format/>
//!
mod decoder;
mod sup;

pub use decoder::PgsDecoder;
pub use sup::SupParser;

use thiserror::Error;

/// Error for `Pgs` handling.
#[derive(Debug, Error)]
pub enum PgsError {}
