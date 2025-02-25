use std::process::Command;
use std::fs;
use solana_program::program_error::ProgramError;

pub fn compile(source: &str) -> Result<Vec<u8>, ProgramError> {
    // Create a temporary directory for building
    let temp_dir = tempfile::tempdir().map_err(|_| ProgramError::InvalidInstructionData)?;
    let src_dir = temp_dir.path().join("src");
    fs::create_dir(&src_dir).map_err(|_| ProgramError::InvalidInstructionData)?;

    // Copy program template
    let template = include_str!("program_template.rs");
    fs::write(src_dir.join("lib.rs"), template)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    // Create Cargo.toml
    let cargo_toml = format!(r#"
[package]
name = "lessvm-program"
version = "0.1.7"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
solana-program = "1.17"
    "#);
    fs::write(temp_dir.path().join("Cargo.toml"), cargo_toml)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    // Build the program
    let output = Command::new("cargo")
        .args(["build-sbf"])
        .current_dir(temp_dir.path())
        .output()
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    if !output.status.success() {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Read the compiled program
    let program_path = temp_dir.path()
        .join("target")
        .join("deploy")
        .join("lessvm_program.so");
    
    fs::read(program_path)
        .map_err(|_| ProgramError::InvalidInstructionData)
}

pub fn optimize(bytecode: &[u8], _release: bool) -> Result<Vec<u8>, ProgramError> {
    // For now, just return the bytecode as-is
    Ok(bytecode.to_vec())
}