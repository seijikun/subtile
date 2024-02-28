use std::io::{self, Read};
use thiserror::Error;

/// Error `PDS` (Palette Definition Segment) handling.
#[derive(Debug, Error)]
pub enum Error {
    /// Read `PaletteDefinitionSegment` in a buffer failed.
    #[error("Failed to read buffer with `PaletteDefinitionSegment`")]
    BufferParse(#[source] io::Error),
}

#[derive(Debug, Clone)]
pub struct Palette {
    entries: Vec<PaletteEntry>,
}
impl Palette {
    fn new(entries: Vec<PaletteEntry>) -> Self {
        Self { entries }
    }

    pub fn get(&self, id: u8) -> Option<&PaletteEntry> {
        //HACK with -1, the color id is not necessarily equal to idx + 1
        let idx = id - 1;
        self.entries.get(idx as usize)
    }
}

#[derive(Debug, Clone)]
pub struct PaletteEntry {
    _palette_entry_id: u8,      // Entry number of the palette
    pub luminance: u8,          // Luminance (Y value)
    _color_difference_red: u8,  // Color Difference Red (Cr value)
    _color_difference_blue: u8, // Color Difference Blue (Cb value)
    pub transparency: u8,       // Transparency (Alpha value)
}
#[derive(Debug)]
pub(crate) struct PaletteDefinitionSegment {
    _palette_id: u8,             // ID of the palette
    _palette_version_number: u8, //	Version of this palette within the Epoch
    pub palette: Palette,
}

pub(crate) fn read<R: Read>(
    reader: &mut R,
    segments_size: usize,
) -> Result<PaletteDefinitionSegment, Error> {
    let mut pds_buf = vec![0; segments_size];
    reader
        .read_exact(&mut pds_buf)
        .map_err(Error::BufferParse)?;

    let palette_id = pds_buf[0];
    let palette_version_number = pds_buf[1];

    let nb_palette_entry: usize = (segments_size - 2) / 5;
    assert_eq!((nb_palette_entry * 5) + 2, segments_size);
    let range = 0..nb_palette_entry;
    let palette_entries = range
        .map(|idx| {
            let offset = 2 + (idx * 5);
            PaletteEntry {
                _palette_entry_id: pds_buf[offset],
                luminance: pds_buf[offset + 1],
                _color_difference_red: pds_buf[offset + 2],
                _color_difference_blue: pds_buf[offset + 3],
                transparency: pds_buf[offset + 4],
            }
        })
        .collect();
    Ok(PaletteDefinitionSegment {
        _palette_id: palette_id,
        _palette_version_number: palette_version_number,
        palette: Palette::new(palette_entries),
    })
}
