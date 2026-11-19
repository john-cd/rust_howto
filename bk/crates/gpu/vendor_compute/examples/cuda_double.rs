// TODO Reviewed ??-??-2026.

//! Compiles a CUDA C kernel with NVRTC and checks the GPU result on the host.

use std::error::Error;
use std::io;

use cudarc::driver::CudaContext;
use cudarc::driver::LaunchConfig;
use cudarc::driver::PushKernelArg;
use cudarc::nvrtc::compile_ptx;

const KERNEL_SOURCE: &str = r#"
extern "C" __global__ void double_values(unsigned int* values, unsigned int count) {
    unsigned int index = blockIdx.x * blockDim.x + threadIdx.x;
    if (index < count) {
        values[index] *= 2u;
    }
}
"#;

/// Runs a one-dimensional kernel on the first CUDA device.
///
/// # Errors
///
/// Returns an error when CUDA setup, compilation, dispatch, or readback fails,
/// or when the output differs from the CPU reference.
fn main() -> Result<(), Box<dyn Error>> {
    let context = CudaContext::new(0)?;
    println!("CUDA device: {}", context.name()?);
    let stream = context.default_stream();
    let module = context.load_module(compile_ptx(KERNEL_SOURCE)?)?;
    let kernel = module.load_function("double_values")?;

    let input: Vec<u32> = (0..130).collect();
    let expected: Vec<u32> = input.iter().map(|value| value * 2).collect();
    let count = u32::try_from(input.len())?;
    let mut values = stream.clone_htod(&input)?;

    // SAFETY: The launch arguments match the CUDA C signature. The stream
    // orders the copies and kernel, and the bounds check protects the tail.
    unsafe {
        stream
            .launch_builder(&kernel)
            .arg(&mut values)
            .arg(&count)
            .launch(LaunchConfig::for_num_elems(count))?;
    }

    let output = stream.clone_dtoh(&values)?;
    if output != expected {
        return Err(io::Error::other("CUDA output differed from the CPU reference").into());
    }
    println!("Verified {} doubled values", output.len());
    Ok(())
}
