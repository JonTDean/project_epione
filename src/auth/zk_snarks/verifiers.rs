use bellman::{groth16::{verify_proof, Proof, PreparedVerifyingKey}, SynthesisError};
use bls12_381::Bls12;
use std::convert::TryInto;
use pairing::group::ff::PrimeField;

pub fn verify_zk_proof(
    pvk: &PreparedVerifyingKey<Bls12>,
    proof: &[u8],
    public_inputs: &[u8],
) -> Result<bool, SynthesisError> {
    // Deserialize the proof
    let proof = Proof::<Bls12>::read(proof)?;

    // Create an instance of our public inputs
    let public_inputs = public_inputs.try_into().expect("Failed to convert public inputs");
    let repr = u128_to_bytes32(u128::from_be_bytes(public_inputs));
    let instance = vec![<Bls12 as pairing::Engine>::Fr::from_repr(repr.into()).expect("Failed to convert public inputs")];

    // Verify the proof
    Ok(verify_proof(pvk, &proof, &instance).is_ok())
}

fn u128_to_bytes32(value: u128) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    bytes[16..32].copy_from_slice(&value.to_be_bytes());
    bytes
}
