use crate::{Boolean, Probability};

/// The result of decoding a repetition-coded boolean.
///
/// `confidence` is the posterior probability for the selected value under the
/// supplied flip model and prior. `corrections` is the number of storage bits
/// that differ from the selected canonical codeword. It is not proof that this
/// many physical flips occurred; if too many bits flipped, majority decoding
/// can select the wrong canonical value.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DecodedBoolean {
    /// False was more probable than true.
    False {
        /// Posterior probability that the original value was false.
        confidence: Probability,
        /// Bits that disagree with the false all-zero codeword.
        corrections: usize,
    },
    /// True was more probable than false.
    True {
        /// Posterior probability that the original value was true.
        confidence: Probability,
        /// Bits that disagree with the true all-one codeword.
        corrections: usize,
    },
    /// Neither value is more probable than the other.
    Ambiguous {
        /// Posterior probability that the original value was false.
        false_probability: Probability,
        /// Posterior probability that the original value was true.
        true_probability: Probability,
    },
}

impl DecodedBoolean {
    /// Returns the selected value, or `None` when the result is tied.
    #[must_use]
    pub const fn value(self) -> Option<Boolean> {
        match self {
            Self::False { .. } => Some(Boolean::False),
            Self::True { .. } => Some(Boolean::True),
            Self::Ambiguous { .. } => None,
        }
    }

    /// Returns the posterior probability of the selected value.
    ///
    /// Ambiguous results return `0.5`.
    #[must_use]
    pub const fn confidence(self) -> Probability {
        match self {
            Self::False { confidence, .. } | Self::True { confidence, .. } => confidence,
            Self::Ambiguous { .. } => Probability::HALF,
        }
    }

    /// Returns the posterior probability assigned to `value`.
    #[must_use]
    pub const fn probability_of(self, value: Boolean) -> Probability {
        match (self, value) {
            (Self::False { confidence, .. }, Boolean::False)
            | (Self::True { confidence, .. }, Boolean::True) => confidence,
            (Self::False { confidence, .. }, Boolean::True)
            | (Self::True { confidence, .. }, Boolean::False) => confidence.complement(),
            (
                Self::Ambiguous {
                    false_probability, ..
                },
                Boolean::False,
            ) => false_probability,
            (
                Self::Ambiguous {
                    true_probability, ..
                },
                Boolean::True,
            ) => true_probability,
        }
    }

    /// Returns bits that disagree with the selected canonical codeword.
    ///
    /// This is `None` for an ambiguous decode.
    #[must_use]
    pub const fn corrections(self) -> Option<usize> {
        match self {
            Self::False { corrections, .. } | Self::True { corrections, .. } => Some(corrections),
            Self::Ambiguous { .. } => None,
        }
    }

    pub(crate) const fn false_with(confidence: Probability, corrections: usize) -> Self {
        Self::False {
            confidence,
            corrections,
        }
    }

    pub(crate) const fn true_with(confidence: Probability, corrections: usize) -> Self {
        Self::True {
            confidence,
            corrections,
        }
    }
}
