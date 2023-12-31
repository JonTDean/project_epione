use bellman::{ConstraintSystem, SynthesisError, LinearCombination};
use bls12_381::Scalar;

fn multiply<CS: ConstraintSystem<Scalar>>(
    cs: &mut CS,
    x: &LinearCombination<Scalar>,
    y: &LinearCombination<Scalar>,
) -> Result<LinearCombination<Scalar>, SynthesisError> {
    // Allocate a new variable for the product
    let z = cs.alloc(|| "product", || Ok(Scalar::zero()))?;

    // Enforce the multiplication constraint
    cs.enforce(
        || "multiplication",
        |lc| lc + x,
        |lc| lc + y,
        |lc| lc + z
    );

    // Convert the Variable to a LinearCombination
      Ok(LinearCombination::zero() + (Scalar::one(), z))
}

pub fn mimc_round<CS: ConstraintSystem<Scalar>>(
    cs: &mut CS,
    x: &LinearCombination<Scalar>,
    k: &LinearCombination<Scalar>,
) -> Result<LinearCombination<Scalar>, SynthesisError> {
    let mut tmp = x.clone() + k;
    let mimc_const_scalar = Scalar::from(42u64); // Parameters for MiMC
    let mimc_rounds: usize = 220;  // Number of rounds 

    // !!! Conversion for Combination to Scalar
    let mimc_const_lc = LinearCombination::zero() + (mimc_const_scalar, CS::one());

    for _ in 0..mimc_rounds {
        tmp = multiply(cs, &tmp, &tmp)?;
        tmp = tmp.clone() + &mimc_const_lc;
    }
    
    Ok(tmp)
}

pub fn hash_function<CS: ConstraintSystem<Scalar>>(
    cs: &mut CS,
    input: LinearCombination<Scalar>,
) -> Result<LinearCombination<Scalar>, SynthesisError> {
    // MiMC requires an additional key input (in zkSNARKs, often set to 0 or some constant)
    let key = LinearCombination::zero();

    // Apply the MiMC round function
    mimc_round(cs, &input, &key)
}
