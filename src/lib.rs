#![forbid(unsafe_code)]
#![warn(missing_docs)]
//! A bit-flip-resilient boolean with model-based confidence estimates.
//!
//! [`Boolean`] is encoded as an all-zero or all-one codeword in a caller
//! selected [`BitStorage`] implementation. This is a repetition code. For one
//! logical bit, it gives the maximum possible Hamming distance for a chosen
//! width and therefore corrects the most arbitrary flips possible at that
//! width.
//!
//! Use [`bits_for_guaranteed_correction`] for an adversarial bound, or
//! [`bits_for_confidence`] when an independent bit-flip probability and a
//! posterior confidence target are known.

mod bit_flip_model;
mod bit_storage;
mod boolean;
mod decoded_boolean;
mod encoded_boolean;
mod error;
mod prior;
mod probability;
mod sizing;
mod storage_recommendation;

pub use bit_flip_model::BitFlipModel;
pub use bit_storage::BitStorage;
pub use boolean::Boolean;
pub use decoded_boolean::DecodedBoolean;
pub use encoded_boolean::EncodedBoolean;
pub use error::{BitFlipModelError, DecodeError, ProbabilityError, SizingError};
pub use prior::BooleanPrior;
pub use probability::Probability;
pub use sizing::{BitRequirement, bits_for_confidence, bits_for_guaranteed_correction};
pub use storage_recommendation::StorageRecommendation;
