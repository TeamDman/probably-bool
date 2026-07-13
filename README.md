# probably-bool

[![Crates.io](https://img.shields.io/crates/v/probably-bool.svg)](https://crates.io/crates/probably-bool)

`probably-bool` stores one logical boolean as a repetition code: every usable
bit in the chosen storage contains the same value. A received word is decoded
by majority vote and can be given an exact Bayesian confidence under an
independent bit-flip model.

For a single logical bit, this is deliberately a repetition code rather than a
Hamming code. Its two codewords are all zeroes and all ones, so their Hamming
distance equals the full storage width. It therefore corrects up to
`floor((width - 1) / 2)` arbitrary bit flips, which is optimal for two
codewords of that width. Hamming/SECDED codes become more space-efficient when
packing multiple data bits; they are a sensible future extension, but are not
the strongest first codec for one boolean.

```rust
use probably_bool::{BitFlipModel, Boolean, EncodedBoolean};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let original = Boolean::True;
    let encoded: EncodedBoolean<u64> = original.encode();

    // In a real program this is the word read back from fallible storage.
    let received = EncodedBoolean::from_storage(u64::MAX ^ 0b101);
    let model = BitFlipModel::new(0.01)?;
    let decoded = received.decode(&model)?;

    assert_eq!(decoded.value(), Some(Boolean::True));
    assert!(decoded.confidence().as_f64() > 0.999);
    Ok(())
}
```

For a hard limit of three flipped bits, seven bits are sufficient for
deterministic correction: use `bits_for_guaranteed_correction(3)`. For a
probabilistic sizing target, use `bits_for_confidence`, which combines an
independent bit-flip rate with a requested posterior confidence.

## Examples

Each example answers one question and can be run with `cargo run --example <name>`:

* [`basic_decode`](examples/basic_decode.rs) — encode a boolean, inject faults,
  and decode it with a posterior confidence.
* [`sizing`](examples/sizing.rs) — choose a width for a deterministic or
  probability-model-based requirement.
* [`flip_distribution`](examples/flip_distribution.rs) — inspect the binomial
  distribution of independent bit-flip counts.
* [`custom_storage`](examples/custom_storage.rs) — implement `BitStorage` for
  exactly seven usable bits.

## Guarantees and assumptions

* A width of `2 * flips + 1` corrects any pattern of at most `flips` errors.
* Bayesian confidence assumes independent flips with the supplied per-bit
  probability and a uniform prior unless `decode_with_prior` is used.
* Confidence is a model-based posterior, not a physical guarantee. If faults
  are correlated, adversarial, or the flip rate is wrong, use the deterministic
  correction bound instead.
* `u8` through `u128` and fixed-size byte/word arrays are supported out of the
  box. Implement `BitStorage` to use another fixed-width representation.

## Publishing

Run the release checks and validate the crates.io package locally:

```powershell
.\publish.ps1 -DryRun
```

After authenticating Cargo with `cargo login` (or setting
`CARGO_REGISTRY_TOKEN`), publish with:

```powershell
.\publish.ps1
```

## Status

This is a deliberately small first pass: one boolean, one optimal
error-correcting representation, no allocation required for fixed-size
storage, documented probability calculations, and exhaustive small-width
tests. It is not intended for cryptographic fault-injection resistance or as a
replacement for hardware ECC.

## DISCLAIMER

This project was primarily authored by `GPT-5.6 Terra Extra High`.
