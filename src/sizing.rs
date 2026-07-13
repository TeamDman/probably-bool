use crate::{BitFlipModel, Probability, SizingError, StorageRecommendation};

/// A width calculation and a suitable supported storage representation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BitRequirement {
    bits: usize,
    storage: StorageRecommendation,
}

impl BitRequirement {
    pub(crate) const fn new(bits: usize) -> Self {
        Self {
            bits,
            storage: StorageRecommendation::for_bits(bits),
        }
    }

    /// Returns the minimum number of repetition-code bits required.
    #[must_use]
    pub const fn bits(self) -> usize {
        self.bits
    }

    /// Returns a supported storage representation that can hold this width.
    ///
    /// Primitive storage is byte-aligned, so its
    /// [`StorageRecommendation::capacity_bits`] can exceed [`Self::bits`]. The
    /// extra bits are usable repetition-code bits and increase resilience.
    #[must_use]
    pub const fn storage(self) -> StorageRecommendation {
        self.storage
    }

    /// Returns the number of arbitrary flips this width corrects for sure.
    #[must_use]
    pub const fn guaranteed_correction_capacity(self) -> usize {
        self.bits.saturating_sub(1) / 2
    }
}

/// Returns the smallest width that corrects `max_bit_flips` arbitrary flips.
///
/// The result is `2 * max_bit_flips + 1`, the standard minimum-distance bound
/// for a two-codeword repetition code. This is a deterministic guarantee and
/// does not require a probability model.
pub fn bits_for_guaranteed_correction(max_bit_flips: usize) -> Result<BitRequirement, SizingError> {
    let bits = max_bit_flips
        .checked_mul(2)
        .and_then(|value| value.checked_add(1))
        .ok_or(SizingError::ArithmeticOverflow)?;
    Ok(BitRequirement::new(bits))
}

/// Returns a width that meets a Bayesian confidence target after observed flips.
///
/// The calculation assumes a uniform false/true prior and the supplied
/// independent bit-flip model. The returned width guarantees that if the
/// received word differs from the selected canonical codeword by at most
/// `max_observed_flips`, its posterior confidence is at least `confidence`.
///
/// This is a model-based statement. For a hard adversarial bound, use
/// [`bits_for_guaranteed_correction`] instead.
pub fn bits_for_confidence(
    max_observed_flips: usize,
    model: &BitFlipModel,
    confidence: Probability,
) -> Result<BitRequirement, SizingError> {
    let confidence = confidence.as_f64();
    if confidence <= 0.5 {
        return Err(SizingError::ConfidenceMustExceedOneHalf);
    }
    if confidence >= 1.0 {
        return Err(SizingError::ConfidenceMustBeLessThanOne);
    }
    if model.rate() == 0.0 {
        return Err(SizingError::ZeroBitFlipRate);
    }

    // For e disagreements, posterior(candidate) is
    // 1 / (1 + (p / (1 - p))^(bits - 2e)). Solve that inequality directly.
    let log_ratio = (model.rate() / (1.0 - model.rate())).ln();
    let desired_log_ratio = ((1.0 - confidence) / confidence).ln();
    let margin = desired_log_ratio / log_ratio;
    let required_margin = ceiling_ignoring_roundoff(margin).max(1.0);
    if !required_margin.is_finite() || required_margin > usize::MAX as f64 {
        return Err(SizingError::ArithmeticOverflow);
    }

    let base = max_observed_flips
        .checked_mul(2)
        .ok_or(SizingError::ArithmeticOverflow)?;
    let bits = base
        .checked_add(required_margin as usize)
        .ok_or(SizingError::ArithmeticOverflow)?;
    Ok(BitRequirement::new(bits))
}

fn ceiling_ignoring_roundoff(value: f64) -> f64 {
    let floor = value.floor();
    // The two sides of the sizing equation can be mathematically identical
    // while their independently rounded f64 representations differ by a few
    // ulps. Do not allocate a whole extra bit for that artifact.
    if value - floor <= 1e-12 {
        floor
    } else {
        value.ceil()
    }
}
