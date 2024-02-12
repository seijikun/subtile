use crate::image::ImageSize;

/// Store Image data directly from `PGS`.
#[derive(Clone)]
pub struct RleEncodedImage {
    width: u16,
    height: u16,
    raw: Vec<u8>,
}

impl RleEncodedImage {
    /// Create a `RleEncodedImage` from [`SupParser`]
    ///
    /// [`SupParser`]: super::sup::SupParser
    #[must_use]
    pub fn new(width: u16, height: u16, raw: Vec<u8>) -> Self {
        Self { width, height, raw }
    }
}

impl ImageSize for RleEncodedImage {
    fn width(&self) -> u32 {
        u32::from(self.width)
    }
    fn height(&self) -> u32 {
        u32::from(self.height)
    }
}
