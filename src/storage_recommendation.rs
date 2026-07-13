/// A storage representation large enough to hold a required number of bits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StorageRecommendation {
    /// Use `u8`.
    U8,
    /// Use `u16`.
    U16,
    /// Use `u32`.
    U32,
    /// Use `u64`.
    U64,
    /// Use `u128`.
    U128,
    /// Use `[u8; bytes]` or `Vec<u8>`.
    ByteArray {
        /// Number of bytes required to contain the requested bit count.
        bytes: usize,
    },
}

impl StorageRecommendation {
    /// Selects the smallest built-in supported representation for `bits`.
    ///
    /// Zero bits select `u8`, because the supported primitive representations
    /// all have at least eight bits.
    #[must_use]
    pub const fn for_bits(bits: usize) -> Self {
        match bits {
            0..=8 => Self::U8,
            9..=16 => Self::U16,
            17..=32 => Self::U32,
            33..=64 => Self::U64,
            65..=128 => Self::U128,
            _ => Self::ByteArray {
                bytes: bits.div_ceil(8),
            },
        }
    }

    /// Returns the number of usable bits in the recommended representation.
    #[must_use]
    pub const fn capacity_bits(self) -> usize {
        match self {
            Self::U8 => 8,
            Self::U16 => 16,
            Self::U32 => 32,
            Self::U64 => 64,
            Self::U128 => 128,
            Self::ByteArray { bytes } => bytes.saturating_mul(8),
        }
    }
}
