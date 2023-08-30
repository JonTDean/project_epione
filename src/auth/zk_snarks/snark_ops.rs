use std::{path::Path, fs::{File, create_dir_all }, io::{Read, Write}};
use bellman::groth16::{Parameters, PreparedVerifyingKey, prepare_verifying_key};
use bls12_381::Bls12;

use super::PARAMS_PATH;

pub fn read_params_from_file() -> Result<Parameters<Bls12>, std::io::Error> {
    // Open the file
    let mut file = File::open(PARAMS_PATH)?;

    // Read the file's content
    let mut params_bytes = Vec::new();
    file.read_to_end(&mut params_bytes)?;

    // Deserialize the parameters
    let params = Parameters::read(&params_bytes[..], true)?;
    
    Ok(params)
}

pub fn get_prepared_vk_from_params(params: &Parameters<Bls12>) -> PreparedVerifyingKey<Bls12> {
    prepare_verifying_key(&params.vk)
}

pub fn save_params_to_file(params: &Parameters<Bls12>, path: &str) -> std::io::Result<()> {
    // Serialize the parameters to bytes
    let mut params_bytes = vec![];
    params.write(&mut params_bytes)?;

    // Check if the directory exists, and if not, create it
    let path_obj = Path::new(path);
    if let Some(parent) = path_obj.parent() {
        if !parent.exists() {
            create_dir_all(parent)?;
        }
    }

    // Save parameters to the specified file
    let mut file = File::create(path)?;
    file.write_all(&params_bytes)?;

    Ok(())
}
