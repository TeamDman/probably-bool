use crate::{Boolean, Probability};

/// Prior probability distribution over a [`Boolean`] before reading storage.
///
/// [`Self::UNIFORM`] is used by [`crate::EncodedBoolean::decode`]. Supply a
/// domain-specific prior through
/// [`crate::EncodedBoolean::decode_with_prior`] when one value is known to be
/// more common than the other.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BooleanPrior {
    true_probability: Probability,
}

impl BooleanPrior {
    /// A prior that assigns equal probability to false and true.
    pub const UNIFORM: Self = Self {
        true_probability: Probability::HALF,
    };

    /// Creates a prior from the probability that the original value is true.
    #[must_use]
    pub const fn new(true_probability: Probability) -> Self {
        Self { true_probability }
    }

    /// Returns the prior probability assigned to `value`.
    #[must_use]
    pub const fn probability_of(self, value: Boolean) -> Probability {
        match value {
            Boolean::False => self.true_probability.complement(),
            Boolean::True => self.true_probability,
        }
    }
}
