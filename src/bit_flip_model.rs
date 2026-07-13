use crate::{BitFlipModelError, Probability};

/// An independent, identically distributed bit-flip model.
///
/// The model assumes every stored bit flips independently with the same
/// probability. Rates at or above `0.5` are rejected: at `0.5` a bit contains
/// no information, and above `0.5` the physical representation should be
/// inverted before decoding.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BitFlipModel {
    rate: f64,
}

impl BitFlipModel {
    /// Creates a model with a per-bit flip probability in `0.0..0.5`.
    ///
    /// A rate of zero is useful for deterministic validation. A mixed received
    /// word then produces [`crate::DecodeError::ImpossibleObservation`].
    pub fn new(rate: f64) -> Result<Self, BitFlipModelError> {
        if rate.is_finite() && (0.0..0.5).contains(&rate) {
            Ok(Self { rate })
        } else {
            Err(BitFlipModelError { rate })
        }
    }

    /// Returns the assumed independent probability that one bit flips.
    #[must_use]
    pub const fn rate(self) -> f64 {
        self.rate
    }

    /// Returns the probability of exactly `flips` errors among `bits` bits.
    ///
    /// This is the binomial distribution over error counts, not the
    /// probability of one particular error pattern. Values too small for an
    /// `f64` are returned as zero.
    #[must_use]
    pub fn probability_of_exactly(self, flips: usize, bits: usize) -> Probability {
        if flips > bits {
            return Probability::ZERO;
        }

        if self.rate == 0.0 {
            return if flips == 0 {
                Probability::ONE
            } else {
                Probability::ZERO
            };
        }

        let log_choose = log_binomial_coefficient(bits, flips);
        let log_probability = log_choose
            + (flips as f64) * self.rate.ln()
            + ((bits - flips) as f64) * (1.0 - self.rate).ln();

        Probability::unchecked(log_probability.exp().min(1.0))
    }

    /// Returns the probability of at most `flips` errors among `bits` bits.
    #[must_use]
    pub fn probability_of_at_most(self, flips: usize, bits: usize) -> Probability {
        let maximum = flips.min(bits);
        let probability = (0..=maximum)
            .map(|count| self.probability_of_exactly(count, bits).as_f64())
            .sum::<f64>()
            .min(1.0);
        Probability::unchecked(probability)
    }

    pub(crate) fn log_pattern_likelihood(self, flips: usize, bits: usize) -> f64 {
        debug_assert!(flips <= bits);

        if self.rate == 0.0 {
            if flips == 0 { 0.0 } else { f64::NEG_INFINITY }
        } else {
            (flips as f64) * self.rate.ln() + ((bits - flips) as f64) * (1.0 - self.rate).ln()
        }
    }
}

fn log_binomial_coefficient(bits: usize, flips: usize) -> f64 {
    let flips = flips.min(bits - flips);
    (1..=flips)
        .map(|index| ((bits - flips + index) as f64).ln() - (index as f64).ln())
        .sum()
}
