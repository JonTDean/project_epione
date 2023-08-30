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
    let public_inputs: [u8; 32] = public_inputs.try_into().expect("Failed to convert public inputs");
    let repr: [u8; 32] = public_inputs;
    let instance = vec![<Bls12 as pairing::Engine>::Fr::from_repr(repr).expect("Failed to convert public inputs")];

    // Verify the proof
    Ok(verify_proof(pvk, &proof, &instance).is_ok())
}