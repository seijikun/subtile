use super::PgsError;
use std::fmt;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SegmentTypeCode {
    Pds = 0x14,
    Ods = 0x15,
    Pcs = 0x16,
    Wds = 0x17,
    End = 0x80,
}

impl TryFrom<u8> for SegmentTypeCode {
    type Error = PgsError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x14 => Ok(Self::Pds),
            0x15 => Ok(Self::Ods),
            0x16 => Ok(Self::Pcs),
            0x17 => Ok(Self::Wds),
            0x80 => Ok(Self::End),
            _ => Err(PgsError::SegmentInvalidTypeCode { value }),
        }
    }
}
impl From<SegmentTypeCode> for u8 {
    fn from(val: SegmentTypeCode) -> Self {
        val as Self
    }
}
impl From<SegmentTypeCode> for &'static str {
    fn from(val: SegmentTypeCode) -> Self {
        match val {
            SegmentTypeCode::Pds => "PDS",
            SegmentTypeCode::Ods => "ODS",
            SegmentTypeCode::Pcs => "PCS",
            SegmentTypeCode::Wds => "WDS",
            SegmentTypeCode::End => "END",
        }
    }
}
impl fmt::Debug for SegmentTypeCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hex: u8 = (*self).into();
        write!(f, "{hex:#02x}-{self}")
    }
}
impl fmt::Display for SegmentTypeCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let friendly: &str = (*self).into();
        write!(f, "{friendly}")
    }
}
