//! Compiles the Rust shader and validates the generated Vulkan SPIR-V.

use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::path::PathBuf;

use spirv_builder::SpirvBuilder;
use spirv_tools::TargetEnv;
use spirv_tools::val::Validator as _;
use spirv_tools::val::{self};

/// Builds and validates the shader, then copies it to the host's output directory.
///
/// # Errors
///
/// Returns an error if compilation, SPIR-V validation, or copying fails.
fn main() -> Result<(), Box<dyn Error>> {
    for path in ["shader/src", "shader/Cargo.toml", "shader/Cargo.lock"] {
        println!("cargo:rerun-if-changed={path}");
    }
    let result = SpirvBuilder::new("shader", "spirv-unknown-vulkan1.1").build()?;
    let bytes = fs::read(result.module.unwrap_single())?;
    let (words, remainder) = bytes.as_chunks::<4>();
    if !remainder.is_empty() {
        return Err(io::Error::other("SPIR-V byte length is not a multiple of four").into());
    }
    let words: Vec<u32> = words.iter().copied().map(u32::from_le_bytes).collect();
    val::create(Some(TargetEnv::Vulkan_1_1)).validate(&words, None)?;
    let out =
        env::var_os("OUT_DIR").ok_or_else(|| io::Error::other("Cargo did not set OUT_DIR"))?;
    fs::write(PathBuf::from(out).join("triangle.spv"), bytes)?;
    Ok(())
}
