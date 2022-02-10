use crate::double::Double;

impl Double {
    /// Returns the minimum of the two numbers.
    pub fn min(self, other: Self) -> Self {
        match self.partial_cmp(&other) {
            Some(ordering) => match ordering {
                core::cmp::Ordering::Less | core::cmp::Ordering::Equal => self,
                core::cmp::Ordering::Greater => other,
            },
            None => other,
        }
    }

    pub fn max(self, other: Self) -> Self {
        match self.partial_cmp(&other) {
            Some(ordering) => match ordering {
                core::cmp::Ordering::Less | core::cmp::Ordering::Equal => other,
                core::cmp::Ordering::Greater => self,
            },
            None => other,
        }
    }

    /// Raw transmutation from `u128`.
    pub fn from_bits(b: u128) -> Self { unsafe { core::mem::transmute::<u128, Self>(b) } }

    /// Raw transmutation to `u128`.
    pub fn to_bits(self) -> u128 { unsafe { core::mem::transmute::<Self, u128>(self) } }

    /// Create a floating point value from its representation as a byte array in
    /// big endian.
    pub fn from_be_bytes(bytes: [u8; 16]) -> Double {
        Double::from_bits(u128::from_be_bytes(bytes))
    }

    /// Create a floating point value from its representation as a byte array in
    /// little endian.
    pub fn from_le_bytes(bytes: [u8; 16]) -> Double {
        Double::from_bits(u128::from_le_bytes(bytes))
    }

    /// Create a floating point value from its representation as a byte array in
    /// native endian.
    pub fn from_ne_bytes(bytes: [u8; 16]) -> Double {
        Double::from_bits(u128::from_ne_bytes(bytes))
    }

    /// Return the memory representation of this floating point number as a byte
    /// array in big-endian (network) byte order.
    pub fn to_be_bytes(self) -> [u8; 16] { self.to_bits().to_be_bytes() }

    /// Return the memory representation of this floating point number as a byte
    /// array in little-endian byte order.
    pub fn to_le_bytes(self) -> [u8; 16] { self.to_bits().to_le_bytes() }

    /// Return the memory representation of this floating point number as a byte
    /// array in native byte order.
    pub fn to_ne_bytes(self) -> [u8; 16] { self.to_bits().to_ne_bytes() }
}
