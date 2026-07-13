use probably_bool::{BitFlipModel, Boolean, EncodedBoolean};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let original = Boolean::True;
    let stored: EncodedBoolean<u8> = original.encode();

    // Simulate two flipped bits while the word is stored or transmitted.
    let received = EncodedBoolean::from_storage(stored.into_storage() ^ 0b0010_0001);
    let decoded = received.decode(&BitFlipModel::new(0.1)?)?;

    println!("decoded value: {:?}", decoded.value());
    println!(
        "posterior confidence: {:.4}%",
        decoded.confidence().as_f64() * 100.0
    );
    println!(
        "bits differing from the decoded codeword: {:?}",
        decoded.corrections()
    );

    Ok(())
}
