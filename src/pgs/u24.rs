use std::fmt::Debug;

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct u24([u8; 3]);

impl u24 {
    pub const fn to_u32(self) -> u32 {
        let Self([a, b, c]) = self;
        u32::from_be_bytes([0, a, b, c])
    }
}

impl From<[u8; 3]> for u24 {
    fn from(value: [u8; 3]) -> Self {
        Self(value)
    }
}

impl Debug for u24 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self.to_u32();
        write!(f, "{value}")
    }
}
