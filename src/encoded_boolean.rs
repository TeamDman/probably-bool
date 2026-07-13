use crate::{
    BitFlipModel, BitStorage, Boolean, BooleanPrior, DecodeError, DecodedBoolean, Probability,
};

/// A boolean stored as an all-zero or all-one repetition-code word.
///
/// The storage type determines the width. For example, `EncodedBoolean<u64>`
/// has a 64-bit codeword, while `EncodedBoolean<[u8; 17]>` has 136 bits. The
/// representation is transparent to callers through [`Self::storage`] and
/// [`Self::into_storage`], allowing it to be written to existing storage.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EncodedBoolean<S> {
    storage: S,
}

impl<S> EncodedBoolean<S> {
    /// Wraps a received or manually constructed storage value for decoding.
    #[must_use]
    pub const fn from_storage(storage: S) -> Self {
        Self { storage }
    }

    /// Borrows the raw codeword storage.
    #[must_use]
    pub const fn storage(&self) -> &S {
        &self.storage
    }

    /// Returns the raw codeword storage.
    #[must_use]
    pub fn into_storage(self) -> S {
        self.storage
    }
}

impl<S: BitStorage> EncodedBoolean<S> {
    /// Returns the number of usable repetition-code bits.
    #[must_use]
    pub fn bit_len(&self) -> usize {
        self.storage.bit_len()
    }

    /// Returns the number of received one bits.
    #[must_use]
    pub fn count_ones(&self) -> usize {
        self.storage.count_ones()
    }

    /// Returns the number of arbitrary bit flips this width corrects for sure.
    ///
    /// This is `floor((bit_len - 1) / 2)`. A zero-bit representation corrects
    /// no errors and has no distinguishable codewords.
    #[must_use]
    pub fn guaranteed_correction_capacity(&self) -> usize {
        self.bit_len().saturating_sub(1) / 2
    }

    /// Decodes using a uniform prior over false and true.
    ///
    /// See [`Self::decode_with_prior`] to provide a domain-specific prior.
    pub fn decode(&self, model: &BitFlipModel) -> Result<DecodedBoolean, DecodeError> {
        self.decode_with_prior(model, BooleanPrior::UNIFORM)
    }

    /// Decodes with a supplied independent flip model and prior distribution.
    ///
    /// The returned confidence is the Bayesian posterior probability of the
    /// selected boolean. A tie is represented by [`DecodedBoolean::Ambiguous`]
    /// rather than arbitrarily choosing a value.
    pub fn decode_with_prior(
        &self,
        model: &BitFlipModel,
        prior: BooleanPrior,
    ) -> Result<DecodedBoolean, DecodeError> {
        let bits = self.bit_len();
        let false_errors = self.count_ones();
        let true_errors = bits - false_errors;

        if model.rate() == 0.0 {
            return self.decode_without_flips(bits, false_errors, prior);
        }

        let false_probability = prior.probability_of(Boolean::False).as_f64();
        let true_probability = prior.probability_of(Boolean::True).as_f64();
        let false_log = log_with_prior(
            model.log_pattern_likelihood(false_errors, bits),
            false_probability,
        );
        let true_log = log_with_prior(
            model.log_pattern_likelihood(true_errors, bits),
            true_probability,
        );

        Ok(decode_log_likelihoods(
            false_log,
            true_log,
            false_errors,
            true_errors,
        ))
    }

    fn decode_without_flips(
        &self,
        bits: usize,
        false_errors: usize,
        prior: BooleanPrior,
    ) -> Result<DecodedBoolean, DecodeError> {
        if bits == 0 {
            return Ok(decode_log_likelihoods(
                log_with_prior(0.0, prior.probability_of(Boolean::False).as_f64()),
                log_with_prior(0.0, prior.probability_of(Boolean::True).as_f64()),
                0,
                0,
            ));
        }

        if false_errors == 0 {
            return if prior.probability_of(Boolean::False) == Probability::ZERO {
                Err(DecodeError::ImpossibleObservation {
                    bits,
                    ones: false_errors,
                })
            } else {
                Ok(DecodedBoolean::false_with(Probability::ONE, 0))
            };
        }

        if false_errors == bits {
            return if prior.probability_of(Boolean::True) == Probability::ZERO {
                Err(DecodeError::ImpossibleObservation {
                    bits,
                    ones: false_errors,
                })
            } else {
                Ok(DecodedBoolean::true_with(Probability::ONE, 0))
            };
        }

        Err(DecodeError::ImpossibleObservation {
            bits,
            ones: false_errors,
        })
    }
}

fn log_with_prior(likelihood: f64, prior: f64) -> f64 {
    if prior == 0.0 {
        f64::NEG_INFINITY
    } else {
        likelihood + prior.ln()
    }
}

fn decode_log_likelihoods(
    false_log: f64,
    true_log: f64,
    false_errors: usize,
    true_errors: usize,
) -> DecodedBoolean {
    if false_log == true_log {
        return DecodedBoolean::Ambiguous {
            false_probability: Probability::HALF,
            true_probability: Probability::HALF,
        };
    }

    if false_log > true_log {
        let confidence = Probability::unchecked(sigmoid(false_log - true_log));
        DecodedBoolean::false_with(confidence, false_errors)
    } else {
        let confidence = Probability::unchecked(sigmoid(true_log - false_log));
        DecodedBoolean::true_with(confidence, true_errors)
    }
}

fn sigmoid(log_odds: f64) -> f64 {
    if log_odds >= 0.0 {
        1.0 / (1.0 + (-log_odds).exp())
    } else {
        let odds = log_odds.exp();
        odds / (1.0 + odds)
    }
}
