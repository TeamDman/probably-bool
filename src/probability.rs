use crate::ProbabilityError;

/// A finite probability in the inclusive range `0.0..=1.0`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Probability(f64);

impl Probability {
    /// Probability zero.
    pub const ZERO: Self = Self(0.0);
    /// Probability one half.
    pub const HALF: Self = Self(0.5);
    /// Probability one.
    pub const ONE: Self = Self(1.0);

    /// Creates a probability after validating that it is finite and in range.
    pub fn new(value: f64) -> Result<Self, ProbabilityError> {
        if value.is_finite() && (0.0..=1.0).contains(&value) {
            Ok(Self(value))
        } else {
            Err(ProbabilityError { value })
        }
    }

    /// Returns the probability as an `f64`.
    #[must_use]
    pub const fn as_f64(self) -> f64 {
        self.0
    }

    /// Returns the probability of the complementary event.
    #[must_use]
    pub const fn complement(self) -> Self {
        Self(1.0 - self.0)
    }

    pub(crate) const fn unchecked(value: f64) -> Self {
        Self(value)
    }
}
