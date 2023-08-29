use std::{path::Path, fmt, fs::{File, create_dir_all}, io::{Read, Write}};
use bellman::groth16::{Parameters, PreparedVerifyingKey, prepare_verifying_key};
use bls12_381::Bls12;

use super::PARAMS_PATH;

pub struct DebuggableParameters(pub Parameters<Bls12>);

impl fmt::Debug for DebuggableParameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Extract fields from the VerifyingKey
        let vk = &self.0.vk;

        // Attempt to create representations for curve points. 
        // This is a placeholder; the actual methods might differ.
        // We'll attempt to use Debug for simplicity, but in practice you might need more specific methods.
        let alpha_g1_repr = format!("{:?}", vk.alpha_g1);
        let beta_g1_repr = format!("{:?}", vk.beta_g1);
        let beta_g2_repr = format!("{:?}", vk.beta_g2);
        let gamma_g2_repr = format!("{:?}", vk.gamma_g2);
        let delta_g1_repr = format!("{:?}", vk.delta_g1);
        let delta_g2_repr = format!("{:?}", vk.delta_g2);
        let ic_repr = vk.ic.iter().map(|point| format!("{:?}", point)).collect::<Vec<String>>().join(", ");

        writeln!(f, "Parameters<Bls12>:")?;
        writeln!(f, "  Verification Key (vk):")?;
        writeln!(f, "    alpha_g1: {}", alpha_g1_repr)?;
        writeln!(f, "    beta_g1: {}", beta_g1_repr)?;
        writeln!(f, "    beta_g2: {}", beta_g2_repr)?;
        writeln!(f, "    gamma_g2: {}", gamma_g2_repr)?;
        writeln!(f, "    delta_g1: {}", delta_g1_repr)?;
        writeln!(f, "    delta_g2: {}", delta_g2_repr)?;
        writeln!(f, "    ic: [{}]", ic_repr)?;

        // Print the other fields of Parameters<Bls12>
        writeln!(f, "  h: {:#?}", self.0.h)?;
        writeln!(f, "  l: {:#?}", self.0.l)?;
        writeln!(f, "  a: {:#?}", self.0.a)?;
        writeln!(f, "  b_g1: {:#?}", self.0.b_g1)?;
        writeln!(f, "  b_g2: {:#?}", self.0.b_g2)?;

        Ok(())
    }
}



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
