// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Validates a WGSL shader with `naga` and translates it to SPIR-V.

use naga::back::spv;
use naga::front::wgsl;
use naga::valid::Capabilities;
use naga::valid::ValidationFlags;
use naga::valid::Validator;

/// Shared `shader` used by this module.
const SHADER: &str = "
@vertex
/// # Arguments
/// * `index` - Position used to identify the element affected in the current ordered collection.
fn vertex_main(@builtin(vertex_index) index: u32) -> @builtin(position) vec4<f32> {
    let x = f32(i32(index) - 1);
    let y = f32(i32(index & 1u) * 2 - 1);
    return vec4<f32>(x, y, 0.0, 1.0);
}

@fragment
fn fragment_main() -> @location(0) vec4<f32> {
    return vec4<f32>(0.2, 0.5, 0.9, 1.0);
}
";

/// Parses and validates the shader, then returns its SPIR-V words.
///
/// No GPU, window, or driver is involved: `naga` is a pure Rust translator.
fn compile_to_spirv() -> anyhow::Result<Vec<u32>> {
    let module =
        wgsl::parse_str(SHADER).map_err(|error| anyhow::anyhow!(error.emit_to_string(SHADER)))?;
    let info = Validator::new(ValidationFlags::all(), Capabilities::all()).validate(&module)?;
    let words = spv::write_vec(&module, &info, &spv::Options::default(), None)?;
    Ok(words)
}

/// Compiles the shader and reports the size of the SPIR-V binary.
fn main() -> anyhow::Result<()> {
    let words = compile_to_spirv()?;
    println!("SPIR-V binary: {} words", words.len());
    Ok(())
}
// ANCHOR_END: example

/// Runs the example through its program entry point.
///
/// # Errors
/// Returns an error if the example operation fails.
pub fn run() -> anyhow::Result<()> {
    main()
}

/// Validates a WGSL shader with `naga` and translates it to SPIR-V.
///
/// # Errors
/// Returns an error if an operation in the example fails.
/// Returns an error if a step of the example fails.

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point.
    #[test]
    fn test_main_entry_point() -> anyhow::Result<()> {
        main()
    }
    /// Verifies the output starts with the SPIR-V magic number.

    #[test]
    fn emits_a_spirv_module() -> anyhow::Result<()> {
        let words = compile_to_spirv()?;
        assert_eq!(words.first(), Some(&0x0723_0203));
        Ok(())
    }

    #[test]
    fn rejects_invalid_wgsl() {
        assert!(wgsl::parse_str("fn broken( {").is_err());
    }
}
