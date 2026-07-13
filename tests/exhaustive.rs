use probably_bool::{BitFlipModel, Boolean, DecodedBoolean, EncodedBoolean};

const MODEL_RATE: f64 = 0.01;

#[test]
fn every_u8_codeword_has_the_expected_decode_for_both_original_values() {
    let model = BitFlipModel::new(MODEL_RATE).unwrap();
    let false_codeword = Boolean::False.encode::<u8>().into_storage();
    let true_codeword = Boolean::True.encode::<u8>().into_storage();

    assert_eq!(false_codeword, 0);
    assert_eq!(true_codeword, u8::MAX);

    for received in u8::MIN..=u8::MAX {
        let false_flips = received.count_ones() as usize;
        let false_decoded = EncodedBoolean::from_storage(received)
            .decode(&model)
            .unwrap();
        assert_expected_decode(
            false_decoded,
            Boolean::False,
            false_flips,
            u8::BITS as usize,
            &format!("false codeword, received {received:08b}"),
        );

        let true_flips = (!received).count_ones() as usize;
        let true_decoded = EncodedBoolean::from_storage(received)
            .decode(&model)
            .unwrap();
        assert_expected_decode(
            true_decoded,
            Boolean::True,
            true_flips,
            u8::BITS as usize,
            &format!("true codeword, received {received:08b}"),
        );
    }
}

// A repetition-code decode depends only on the received Hamming weight. For a
// type with N bits, every raw codeword belongs to one of N + 1 weight classes.
// Checking every class is therefore exhaustive over its observable behavior,
// without impractically iterating all 2^64 or 2^128 codewords.
macro_rules! exhaustive_hamming_weight_classes {
    ($test_name:ident, $integer:ty) => {
        #[test]
        fn $test_name() {
            let model = BitFlipModel::new(MODEL_RATE).unwrap();
            let bits = <$integer>::BITS as usize;

            assert_eq!(Boolean::False.encode::<$integer>().into_storage(), 0);
            assert_eq!(
                Boolean::True.encode::<$integer>().into_storage(),
                <$integer>::MAX
            );

            for flips in 0..=bits {
                let mask: $integer = match flips {
                    0 => 0,
                    count if count == bits => <$integer>::MAX,
                    count => ((1 as $integer) << count) - 1,
                };

                let false_decoded = EncodedBoolean::from_storage(mask).decode(&model).unwrap();
                assert_expected_decode(
                    false_decoded,
                    Boolean::False,
                    flips,
                    bits,
                    &format!("false codeword, {flips} flips"),
                );

                let true_decoded = EncodedBoolean::from_storage(!mask).decode(&model).unwrap();
                assert_expected_decode(
                    true_decoded,
                    Boolean::True,
                    flips,
                    bits,
                    &format!("true codeword, {flips} flips"),
                );
            }
        }
    };
}

exhaustive_hamming_weight_classes!(u8_hamming_weight_classes, u8);
exhaustive_hamming_weight_classes!(u16_hamming_weight_classes, u16);
exhaustive_hamming_weight_classes!(u32_hamming_weight_classes, u32);
exhaustive_hamming_weight_classes!(u64_hamming_weight_classes, u64);
exhaustive_hamming_weight_classes!(u128_hamming_weight_classes, u128);

fn assert_expected_decode(
    decoded: DecodedBoolean,
    original: Boolean,
    flips: usize,
    bits: usize,
    context: &str,
) {
    let expected_value = match flips.cmp(&(bits - flips)) {
        core::cmp::Ordering::Less => Some(original),
        core::cmp::Ordering::Equal => None,
        core::cmp::Ordering::Greater => Some(original.opposite()),
    };
    let expected_corrections = expected_value.map(|_| flips.min(bits - flips));

    assert_eq!(decoded.value(), expected_value, "{context}");
    assert_eq!(decoded.corrections(), expected_corrections, "{context}");
}
