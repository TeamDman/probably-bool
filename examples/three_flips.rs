use probably_bool::{
    BitFlipModel, Boolean, EncodedBoolean, Probability, bits_for_confidence,
    bits_for_guaranteed_correction,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let deterministic = bits_for_guaranteed_correction(3)?;
    println!(
        "Three arbitrary flips need at least {} code bits ({:?}).",
        deterministic.bits(),
        deterministic.storage()
    );

    let model = BitFlipModel::new(0.1)?;
    let probabilistic = bits_for_confidence(3, &model, Probability::new(0.9)?)?;
    println!(
        "At 10% independent flips, 90% posterior confidence after three disagreements needs {} bits.",
        probabilistic.bits()
    );

    // The u8 recommendation supplies eight code bits, one more than the
    // seven-bit minimum. Simulate three faults in an encoded true value.
    let received = EncodedBoolean::from_storage(u8::MAX ^ 0b0010_0101);
    let decoded = received.decode(&model)?;
    assert_eq!(decoded.value(), Some(Boolean::True));
    println!(
        "Decoded with {:.2}% confidence.",
        decoded.confidence().as_f64() * 100.0
    );

    Ok(())
}
