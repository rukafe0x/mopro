use super::garaga_convert::snarkjs_g1_to_garaga;
use garaga_rs::definitions::BN254PrimeField;
use garaga_rs::io::{
    field_elements_from_big_uints, parse_g1_points_from_flattened_field_elements_list,
};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct SnarkJsVerificationKey {
    pub protocol: String,
    pub curve: String,
    #[serde(rename = "nPublic")]
    pub n_public: usize,
    pub vk_alpha_1: Vec<String>,
    pub vk_beta_2: Vec<Vec<String>>,
    pub vk_gamma_2: Vec<Vec<String>>,
    pub vk_delta_2: Vec<Vec<String>>,
    #[serde(rename = "IC")]
    pub ic: Vec<Vec<String>>,
}

pub(crate) const SNARKJS_BN128_CURVE: &str = "bn128";
pub(crate) const GROTH16_PROTOCOL: &str = "groth16";

pub(crate) fn parse_snarkjs_vk_json(s: &str) -> Result<SnarkJsVerificationKey, String> {
    let vk: SnarkJsVerificationKey =
        serde_json::from_str(s).map_err(|e| format!("invalid verification key JSON: {e}"))?;
    validate_vk(&vk)?;
    Ok(vk)
}

fn validate_vk(vk: &SnarkJsVerificationKey) -> Result<(), String> {
    if vk.protocol != GROTH16_PROTOCOL {
        return Err(format!(
            "unsupported vk protocol: {} (expected {GROTH16_PROTOCOL})",
            vk.protocol
        ));
    }
    if vk.curve != SNARKJS_BN128_CURVE {
        return Err(format!(
            "unsupported curve: {} (BN254/{SNARKJS_BN128_CURVE} only in v1)",
            vk.curve
        ));
    }
    let expected_ic_len = vk
        .n_public
        .checked_add(1)
        .ok_or_else(|| "nPublic is too large to represent IC length (nPublic + 1)".to_string())?;
    if vk.ic.len() != expected_ic_len {
        return Err(format!(
            "IC length mismatch: expected {} points (nPublic + 1), got {}",
            expected_ic_len,
            vk.ic.len()
        ));
    }
    for (index, coords) in vk.ic.iter().enumerate() {
        let point = snarkjs_g1_to_garaga(coords)
            .map_err(|e| format!("invalid IC point at index {index}: {e}"))?;
        let elements = field_elements_from_big_uints::<BN254PrimeField>(&point.flatten());
        parse_g1_points_from_flattened_field_elements_list(&elements)
            .map_err(|e| format!("invalid IC point at index {index}: {e}"))?;
    }
    Ok(())
}
