use crate::{BitStorage, EncodedBoolean};

/// A logical boolean independent of its fault-tolerant representation.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Boolean {
    /// The logical false value.
    #[default]
    False = 0,
    /// The logical true value.
    True = 1,
}

impl Boolean {
    /// Encodes this value by filling every usable bit of `S` with this value.
    ///
    /// `u64` produces a 64-bit codeword; `[u8; 32]` produces a 256-bit
    /// codeword. A wider codeword has a larger correction guarantee.
    #[must_use]
    pub fn encode<S: BitStorage>(self) -> EncodedBoolean<S> {
        EncodedBoolean::from_storage(S::filled(self))
    }

    /// Returns the opposite boolean value.
    #[must_use]
    pub const fn opposite(self) -> Self {
        match self {
            Self::False => Self::True,
            Self::True => Self::False,
        }
    }
}

impl From<bool> for Boolean {
    fn from(value: bool) -> Self {
        if value { Self::True } else { Self::False }
    }
}

impl From<Boolean> for bool {
    fn from(value: Boolean) -> Self {
        matches!(value, Boolean::True)
    }
}
