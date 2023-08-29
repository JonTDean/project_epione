use std::path::Path;
use bellman::{groth16::{generate_random_parameters, Parameters}, SynthesisError};
use rand::rngs::OsRng;
use bls12_381::{Scalar, Bls12};

use crate::auth::zk_snarks::snark_ops::save_params_to_file;

use super::{registration_circuit::RegistrationCircuit, PARAMS_PATH};

pub fn generate_zksnark_params() -> Result<Parameters<Bls12>, SynthesisError> {
    let rng = &mut OsRng;
    let params = {
        let c = RegistrationCircuit {
            hash: None, // Not needed for parameter generation
            password: Scalar::from(0u64), // Dummy value
        };
        generate_random_parameters(c, rng)?
    };
    Ok(params)
}


pub fn ensure_zksnark_params() {
    // Check if the params file exists
    if !Path::new(PARAMS_PATH).exists() {
        println!("Generating zk-SNARK parameters...");
        let params = generate_zksnark_params().expect("Failed to generate zk-SNARK parameters.");
        
        // Save the parameters to a file
        save_params_to_file(&params, PARAMS_PATH).expect("Failed to save zk-SNARK parameters.");
        println!("zk-SNARK parameters generated and saved.");
    } else {
        println!("zk-SNARK parameters already exist.");
    }
}
