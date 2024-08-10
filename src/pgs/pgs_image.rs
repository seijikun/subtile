use super::pds::Palette;
use crate::image::ImageSize;
use std::io::{ErrorKind, Read};

/// Store Image data directly from `PGS`.
#[derive(Clone)]
pub struct RleEncodedImage {
    width: u16,
    height: u16,
    palette: Palette,
    raw: Vec<u8>,
}

impl RleEncodedImage {
    /// Create a `RleEncodedImage` from [`SupParser`]
    ///
    /// [`SupParser`]: super::sup::SupParser
    #[must_use]
    pub fn new(width: u16, height: u16, palette: Palette, raw: Vec<u8>) -> Self {
        Self {
            width,
            height,
            palette,
            raw,
        }
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

/// Create an iterator over [`RleEncodedImage`] pixels.
impl<'a> IntoIterator for &'a RleEncodedImage {
    type Item = u8;
    type IntoIter = RlePixelIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        RlePixelIterator {
            raw_data: &self.raw,
            current_color: 0,
            nb_remaining_pixels: 0,
        }
    }
}

/// struct to iterate on pixel of an `Rle` image.
pub struct RlePixelIterator<'a> {
    raw_data: &'a [u8],
    current_color: u8,
    nb_remaining_pixels: u16,
}

/// Allow iterate over pixels of image encoded in `Rle`.
impl Iterator for RlePixelIterator<'_> {
    type Item = u8; // PaletteIdx

    fn next(&mut self) -> Option<Self::Item> {
        if self.nb_remaining_pixels > 0 {
            self.nb_remaining_pixels -= 1;
            Some(self.current_color)
        } else if let Some((color, nb_pixel)) = self.read_next_pixel() {
            self.current_color = color;
            self.nb_remaining_pixels = nb_pixel - 1;
            Some(self.current_color)
        } else {
            None // End of pixels
        }
    }
}

impl RlePixelIterator<'_> {
    /// Read next pixel info(color and number of instance).
    fn read_next_pixel(&mut self) -> Option<(u8 /*color */, u16 /*nb_pixels*/)> {
        const MARKER: u8 = 0;
        const COLOR_0: u8 = 0;
        loop {
            let mut color: [u8; 1] = [0; 1];
            let res = self.raw_data.read_exact(&mut color);
            if let Err(err) = res {
                if err.kind() == ErrorKind::UnexpectedEof {
                    return None;
                }
            }

            let next = color[0];
            if next == MARKER {
                let mut l2 = [0; 1];
                self.raw_data.read_exact(&mut l2).unwrap();

                if l2[0] == MARKER {
                    //break; // End of line
                } else {
                    let byte = l2[0];
                    let nb_pixels = match CountMarker::from(byte) {
                        CountMarker::Long => {
                            let mut l3 = [0; 1];
                            self.raw_data.read_exact(&mut l3).unwrap();
                            let count_bytes = [byte & 0b0011_1111, l3[0]];
                            u16::from_be_bytes(count_bytes)
                        }
                        CountMarker::Short => {
                            let count_bits = byte & 0b0011_1111;
                            u16::from(u8::from_be(count_bits))
                        }
                    };

                    let color_marker = ColorMarker::from(byte);
                    let color = match color_marker {
                        ColorMarker::Color0 => COLOR_0,
                        ColorMarker::ColorN => {
                            let mut color = [0; 1];
                            self.raw_data.read_exact(&mut color).unwrap();
                            color[0]
                        }
                    };

                    return Some((color, nb_pixels));
                }
            } else {
                return Some((next, 1));
            }
        }
    }
}

/// Decode the color marker.
enum ColorMarker {
    /// color 0 : black
    Color0,
    /// color N : color define in code
    ColorN,
}
impl From<u8> for ColorMarker {
    fn from(value: u8) -> Self {
        if (value & 0b1000_0000) > 0 {
            Self::ColorN
        } else {
            Self::Color0
        }
    }
}

/// Decode the pixels count marcker.
enum CountMarker {
    /// the number of pixels is between 1 and 63
    Short,
    /// the number of pixels is between 64 and 16383
    Long,
}
impl From<u8> for CountMarker {
    fn from(value: u8) -> Self {
        if (value & 0b0100_0000) > 0 {
            Self::Long
        } else {
            Self::Short
        }
    }
}
