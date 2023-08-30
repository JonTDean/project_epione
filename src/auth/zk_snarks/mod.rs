use std::{fs::{self, create_dir_all}, fmt};

use bellman::groth16::Parameters;
use bls12_381::Bls12;

pub mod registration_circuit;
pub mod mimc_hash;
pub mod snark_setup;
pub mod snark_ops;
pub mod verifiers;

pub const PARAMS_PATH:&str = "data/epione_zsnark.params";


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

impl DebuggableParameters {
    pub fn save_to_files(&self) -> std::io::Result<()> {
        // Ensure the directory exists
        create_dir_all("data/debug")?;

        // Extract fields from the VerifyingKey
        let vk = &self.0.vk;

        // Create representations for curve points
        let alpha_g1_repr = format!("{:?}", vk.alpha_g1);
        let beta_g1_repr = format!("{:?}", vk.beta_g1);
        let beta_g2_repr = format!("{:?}", vk.beta_g2);
        let gamma_g2_repr = format!("{:?}", vk.gamma_g2);
        let delta_g1_repr = format!("{:?}", vk.delta_g1);
        let delta_g2_repr = format!("{:?}", vk.delta_g2);
        let ic_repr = vk.ic.iter().map(|point| format!("{:?}", point)).collect::<Vec<String>>().join(", ");

        // Save to files
        fs::write("data/debug/vk_alpha_g1.txt", alpha_g1_repr)?;
        fs::write("data/debug/vk_beta_g1.txt", beta_g1_repr)?;
        fs::write("data/debug/vk_beta_g2.txt", beta_g2_repr)?;
        fs::write("data/debug/vk_gamma_g2.txt", gamma_g2_repr)?;
        fs::write("data/debug/vk_delta_g1.txt", delta_g1_repr)?;
        fs::write("data/debug/vk_delta_g2.txt", delta_g2_repr)?;
        fs::write("data/debug/vk_ic.txt", ic_repr)?;

        // Save the other fields of Parameters<Bls12> to files
        fs::write("data/debug/params_h.txt", format!("{:?}", self.0.h))?;
        fs::write("data/debug/params_l.txt", format!("{:?}", self.0.l))?;
        fs::write("data/debug/params_a.txt", format!("{:?}", self.0.a))?;
        fs::write("data/debug/params_b_g1.txt", format!("{:?}", self.0.b_g1))?;
        fs::write("data/debug/params_b_g2.txt", format!("{:?}", self.0.b_g2))?;

        Ok(())
    }
}
