use probably_bool::BitFlipModel;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model = BitFlipModel::new(0.01)?;
    let bits = 64;

    println!("Independent 1% flip model across {bits} bits:");
    for flips in 0..=4 {
        let probability = model.probability_of_exactly(flips, bits);
        println!(
            "  exactly {flips} flips: {:.6}%",
            probability.as_f64() * 100.0
        );
    }

    let recoverable = 3;
    let probability = model.probability_of_at_most(recoverable, bits);
    println!(
        "  at most {recoverable} flips: {:.6}%",
        probability.as_f64() * 100.0
    );

    Ok(())
}
