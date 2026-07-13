use probably_bool::{
    BitFlipModel, Probability, bits_for_confidence, bits_for_guaranteed_correction,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let guaranteed = bits_for_guaranteed_correction(3)?;
    println!(
        "Correct any three arbitrary flips with at least {} bits ({:?}).",
        guaranteed.bits(),
        guaranteed.storage()
    );

    let model = BitFlipModel::new(0.1)?;
    let confidence = Probability::new(0.9)?;
    let probabilistic = bits_for_confidence(3, &model, confidence)?;
    println!(
        "At a 10% independent flip rate, 90% posterior confidence after three disagreements needs {} bits ({:?}).",
        probabilistic.bits(),
        probabilistic.storage()
    );

    Ok(())
}
