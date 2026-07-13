use probably_bool::{
    BitFlipModel, BitStorage, Boolean, BooleanPrior, DecodeError, DecodedBoolean, EncodedBoolean,
    Probability, StorageRecommendation, bits_for_confidence, bits_for_guaranteed_correction,
};

#[derive(Clone, Copy)]
struct SevenBits(u8);

impl BitStorage for SevenBits {
    fn bit_len(&self) -> usize {
        7
    }

    fn count_ones(&self) -> usize {
        (self.0 & 0b0111_1111).count_ones() as usize
    }

    fn filled(value: Boolean) -> Self {
        Self(match value {
            Boolean::False => 0,
            Boolean::True => 0b0111_1111,
        })
    }
}

#[test]
fn corrects_three_arbitrary_flips_with_seven_bits() {
    let encoded: EncodedBoolean<u8> = Boolean::True.encode();
    let received = EncodedBoolean::from_storage(encoded.into_storage() ^ 0b0010_0101);
    let decoded = received.decode(&BitFlipModel::new(0.1).unwrap()).unwrap();

    assert_eq!(decoded.value(), Some(Boolean::True));
    assert_eq!(decoded.corrections(), Some(3));
    // `u8` has eight code bits; three disagreements leave a two-bit distance
    // advantage over the opposite codeword.
    assert!((decoded.confidence().as_f64() - 0.987_804_878_048_780_5).abs() < 1e-12);
}

#[test]
fn confidence_sizing_identifies_seven_bits_for_three_flips_at_ninety_percent() {
    let requirement = bits_for_confidence(
        3,
        &BitFlipModel::new(0.1).unwrap(),
        Probability::new(0.9).unwrap(),
    )
    .unwrap();

    assert_eq!(requirement.bits(), 7);
    assert_eq!(requirement.storage(), StorageRecommendation::U8);
}

#[test]
fn deterministic_sizing_identifies_seven_bits_for_three_flips() {
    let requirement = bits_for_guaranteed_correction(3).unwrap();

    assert_eq!(requirement.bits(), 7);
    assert_eq!(requirement.guaranteed_correction_capacity(), 3);
    assert_eq!(requirement.storage(), StorageRecommendation::U8);
}

#[test]
fn user_defined_storage_can_expose_an_exact_non_byte_width() {
    let received = EncodedBoolean::from_storage(SevenBits(0b0111_0100));
    let decoded = received.decode(&BitFlipModel::new(0.1).unwrap()).unwrap();

    assert_eq!(received.bit_len(), 7);
    assert_eq!(decoded.value(), Some(Boolean::True));
    assert!((decoded.confidence().as_f64() - 0.9).abs() < 1e-12);
}

#[test]
fn supports_array_backed_codewords() {
    let encoded: EncodedBoolean<[u8; 32]> = Boolean::False.encode();
    assert_eq!(encoded.bit_len(), 256);
    assert_eq!(encoded.guaranteed_correction_capacity(), 127);
    assert_eq!(encoded.count_ones(), 0);
}

#[test]
fn equal_number_of_zeros_and_ones_is_explicitly_ambiguous() {
    let received = EncodedBoolean::from_storage(0b1111_0000_u8);
    let decoded = received.decode(&BitFlipModel::new(0.01).unwrap()).unwrap();

    assert_eq!(
        decoded,
        DecodedBoolean::Ambiguous {
            false_probability: Probability::HALF,
            true_probability: Probability::HALF,
        }
    );
}

#[test]
fn a_prior_can_break_an_even_width_tie() {
    let received = EncodedBoolean::from_storage(0b1111_0000_u8);
    let prior = BooleanPrior::new(Probability::new(0.9).unwrap());
    let decoded = received
        .decode_with_prior(&BitFlipModel::new(0.01).unwrap(), prior)
        .unwrap();

    assert_eq!(decoded.value(), Some(Boolean::True));
    assert!((decoded.confidence().as_f64() - 0.9).abs() < 1e-12);
}

#[test]
fn flip_count_distribution_is_binomial() {
    let model = BitFlipModel::new(0.1).unwrap();
    let exactly_two = model.probability_of_exactly(2, 3).as_f64();
    let at_most_one = model.probability_of_at_most(1, 3).as_f64();

    assert!((exactly_two - 0.027).abs() < 1e-12);
    assert!((at_most_one - 0.972).abs() < 1e-12);
}

#[test]
fn zero_flip_model_rejects_mixed_observations() {
    let received = EncodedBoolean::from_storage(0b1000_0000_u8);
    let result = received.decode(&BitFlipModel::new(0.0).unwrap());

    assert_eq!(
        result,
        Err(DecodeError::ImpossibleObservation { bits: 8, ones: 1 })
    );
}
