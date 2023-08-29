use bellman::{ Circuit, ConstraintSystem, SynthesisError, LinearCombination };
use bls12_381::Scalar;

use super::mimc_hash::hash_function;

struct RegistrationCircuit {
    // The known hash (public input).
    hash: Option<Scalar>,
    // The secret password (private input).
    password: Scalar,
}

impl Circuit<Scalar> for RegistrationCircuit {
    fn synthesize<CS: ConstraintSystem<Scalar>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        // Allocate variables for the password and hash
        let password_var = cs.alloc(|| "Password", || Ok(self.password))?;
        let hash_var = cs.alloc_input(|| "Hash", || self.hash.ok_or(SynthesisError::AssignmentMissing))?;

        // Let's assume we have a `hash_function` that takes a linear combination 
        // and returns the hashed value as another linear combination.
        // This would be a gadget that you implement or use from an existing library.
        let lc_password = LinearCombination::zero() + password_var;
        let computed_hash = hash_function(cs, lc_password)?;

        // Enforce that the computed hash equals the provided hash
        cs.enforce(
            || "hash(password) = hash",
            |lc| lc.clone() + &computed_hash,
            |lc| lc + CS::one(),
            |lc| lc + hash_var,
        );

        Ok(())
    }
}
