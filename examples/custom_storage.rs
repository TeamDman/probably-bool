use probably_bool::{BitFlipModel, BitStorage, Boolean, EncodedBoolean};

/// A storage type that exposes exactly seven usable bits of a byte.
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stored: EncodedBoolean<SevenBits> = Boolean::True.encode();
    let received = EncodedBoolean::from_storage(SevenBits(stored.into_storage().0 ^ 0b0010_0101));
    let decoded = received.decode(&BitFlipModel::new(0.1)?)?;

    assert_eq!(decoded.value(), Some(Boolean::True));
    println!(
        "seven-bit codeword decoded with {:.2}% confidence",
        decoded.confidence().as_f64() * 100.0
    );

    Ok(())
}
