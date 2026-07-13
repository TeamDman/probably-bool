use crate::Boolean;

/// Storage whose usable bits can hold a repetition-coded [`Boolean`].
///
/// Implement this trait for a custom storage type when the built-in unsigned
/// integers, byte arrays, and word arrays are unsuitable. Padding bits must
/// not be included in [`Self::bit_len`] or [`Self::count_ones`].
pub trait BitStorage: Sized {
    /// Returns the number of usable bits in this storage value.
    fn bit_len(&self) -> usize;

    /// Counts set bits among the usable bits in this storage value.
    fn count_ones(&self) -> usize;

    /// Creates storage with every usable bit set to `value`.
    fn filled(value: Boolean) -> Self;
}

macro_rules! impl_integer_storage {
    ($($integer:ty),+ $(,)?) => {
        $(
            impl BitStorage for $integer {
                fn bit_len(&self) -> usize {
                    <$integer>::BITS as usize
                }

                fn count_ones(&self) -> usize {
                    <$integer>::count_ones(*self) as usize
                }

                fn filled(value: Boolean) -> Self {
                    match value {
                        Boolean::False => 0,
                        Boolean::True => <$integer>::MAX,
                    }
                }
            }
        )+
    };
}

impl_integer_storage!(u8, u16, u32, u64, u128);

macro_rules! impl_array_storage {
    ($element:ty) => {
        impl<const N: usize> BitStorage for [$element; N] {
            fn bit_len(&self) -> usize {
                N.saturating_mul(<$element>::BITS as usize)
            }

            fn count_ones(&self) -> usize {
                self.iter().map(|value| value.count_ones() as usize).sum()
            }

            fn filled(value: Boolean) -> Self {
                let fill = match value {
                    Boolean::False => 0,
                    Boolean::True => <$element>::MAX,
                };
                [fill; N]
            }
        }
    };
}

impl_array_storage!(u8);
impl_array_storage!(u16);
impl_array_storage!(u32);
impl_array_storage!(u64);
impl_array_storage!(u128);
