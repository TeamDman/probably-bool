use core::fmt;

/// An invalid probability value.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProbabilityError {
    /// The rejected value.
    pub value: f64,
}

impl fmt::Display for ProbabilityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "expected a finite probability in 0.0..=1.0, got {}",
            self.value
        )
    }
}

/// An invalid independent bit-flip model.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BitFlipModelError {
    /// The rejected per-bit flip probability.
    pub rate: f64,
}

impl fmt::Display for BitFlipModelError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "expected a finite per-bit flip probability in 0.0..0.5, got {}",
            self.rate
        )
    }
}

/// A received word cannot be explained by the supplied model and prior.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DecodeError {
    /// All candidate original values assign the received word probability zero.
    ImpossibleObservation {
        /// Number of usable bits in the received codeword.
        bits: usize,
        /// Number of one bits in the received codeword.
        ones: usize,
    },
}

impl fmt::Display for DecodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ImpossibleObservation { bits, ones } => write!(
                formatter,
                "the model and prior cannot explain a {}-bit word containing {} one bits",
                bits, ones
            ),
        }
    }
}

/// A sizing request cannot be represented or is not meaningful.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SizingError {
    /// The requested posterior confidence is not strictly above one half.
    ConfidenceMustExceedOneHalf,
    /// A confidence of one cannot be reached with a nonzero flip probability.
    ConfidenceMustBeLessThanOne,
    /// Confidence sizing needs a nonzero flip probability; use deterministic sizing instead.
    ZeroBitFlipRate,
    /// The result does not fit in `usize`.
    ArithmeticOverflow,
}

impl fmt::Display for SizingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConfidenceMustExceedOneHalf => {
                formatter.write_str("confidence must be strictly greater than 0.5")
            }
            Self::ConfidenceMustBeLessThanOne => {
                formatter.write_str("confidence must be strictly less than 1.0")
            }
            Self::ZeroBitFlipRate => formatter.write_str(
                "confidence sizing requires a nonzero flip rate; use guaranteed correction sizing",
            ),
            Self::ArithmeticOverflow => formatter.write_str("requested bit width overflows usize"),
        }
    }
}

impl std::error::Error for ProbabilityError {}

impl std::error::Error for BitFlipModelError {}

impl std::error::Error for DecodeError {}

impl std::error::Error for SizingError {}
