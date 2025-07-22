//! Custom error types.

use thiserror::Error;

/// A type representing errors that are specific to `subtile`. Note that we may
/// normally return `Error`, not `SubError`, which allows to return other
/// kinds of errors from third-party libraries.
#[derive(Debug, Error)]
pub enum SubtileError {
    /// Error with `VobSub`
    #[error("error with VobSub")]
    VobSub(#[from] crate::vobsub::VobSubError),

    /// Error during image dump
    #[error("dump images failed")]
    ImageDump(#[from] crate::image::DumpError),
}
