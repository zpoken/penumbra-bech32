//! Bech32 serializers. Because Bech32 is parameterized by the HRP, this module
//! implements (internal) helper functions that are used in submodules that fill
//! in parameters for various key types.

use bech32::{FromBase32, ToBase32};
// re-exporting allows the convenience methods to be used without referencing
// the underlying bech32 crate.
pub use bech32::{Variant, Variant::*};

/// Convenience method for (general-purpose) Bech32 decoding.
///
/// Works around a bit of awkwardness in the [`bech32`] API.
pub fn decode(
    string: &str,
    expected_hrp: &str,
    expected_variant: Variant,
) -> anyhow::Result<Vec<u8>> {
    let (hrp, data, variant) = bech32::decode(string)?;

    if variant != expected_variant {
        return Err(anyhow::anyhow!(
            "wrong bech32 variant {:?}, expected {:?}",
            variant,
            expected_variant
        ));
    }
    if hrp != expected_hrp {
        return Err(anyhow::anyhow!(
            "wrong bech32 human readable part {}, expected {}",
            hrp,
            expected_hrp
        ));
    }

    Ok(Vec::from_base32(&data).expect("bech32 decoding produces valid base32"))
}

/// Convenience method for (general-purpose) Bech32 encoding.
///
/// Works around a bit of awkwardness in the [`bech32`] API.
/// Panics if the HRP is invalid.
pub fn encode(data: &[u8], hrp: &str, variant: Variant) -> String {
    bech32::encode(hrp, data.to_base32(), variant).expect("HRP should be valid")
}

