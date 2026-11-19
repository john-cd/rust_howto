// TODO Reviewed ??-??-2026.

//! Doubles integers with OpenCL and verifies every value after readback.

use std::error::Error;
use std::io;
use std::ptr;

use opencl3::command_queue::CommandQueue;
use opencl3::context::Context;
use opencl3::device::CL_DEVICE_TYPE_GPU;
use opencl3::device::Device;
use opencl3::device::get_all_devices;
use opencl3::kernel::ExecuteKernel;
use opencl3::kernel::Kernel;
use opencl3::memory::Buffer;
use opencl3::memory::CL_MEM_READ_WRITE;
use opencl3::program::Program;
use opencl3::types::CL_BLOCKING;

const KERNEL_SOURCE: &str = "
__kernel void double_values(__global uint* values, uint count) {
    size_t index = get_global_id(0);
    if (index < count) {
        values[index] *= 2u;
    }
}
";

/// Runs a one-dimensional kernel on the first OpenCL GPU.
///
/// # Errors
///
/// Returns an error when the driver, kernel build, dispatch, or readback fails,
/// or when the output differs from the CPU reference.
fn main() -> Result<(), Box<dyn Error>> {
    let device_id = get_all_devices(CL_DEVICE_TYPE_GPU)?
        .into_iter()
        .next()
        .ok_or_else(|| io::Error::other("no OpenCL GPU device found"))?;
    let device = Device::new(device_id);
    println!("OpenCL device: {}", device.name()?);
    let context = Context::from_device(&device)?;
    let queue = CommandQueue::create_default(&context, 0)?;
    let program = Program::create_and_build_from_source(&context, KERNEL_SOURCE, "")
        .map_err(io::Error::other)?;
    let kernel = Kernel::create(&program, "double_values")?;

    let input: Vec<u32> = (0..130).collect();
    let expected: Vec<u32> = input.iter().map(|value| value * 2).collect();
    let mut output = vec![0; input.len()];
    let count = u32::try_from(input.len())?;

    // SAFETY: The allocation has input.len() writable u32 elements and no host
    // pointer. Transfers block, and both slices cover the whole buffer. The
    // kernel arguments match the OpenCL C signature and its guard bounds writes.
    unsafe {
        let mut values =
            Buffer::<u32>::create(&context, CL_MEM_READ_WRITE, input.len(), ptr::null_mut())?;
        queue.enqueue_write_buffer(&mut values, CL_BLOCKING, 0, &input, &[])?;
        let finished = ExecuteKernel::new(&kernel)
            .set_arg(&values)
            .set_arg(&count)
            .set_global_work_size(input.len().next_multiple_of(64))
            .enqueue_nd_range(&queue)?;
        queue.enqueue_read_buffer(&values, CL_BLOCKING, 0, &mut output, &[finished.get()])?;
    }

    if output != expected {
        return Err(io::Error::other("OpenCL output differed from the CPU reference").into());
    }
    println!("Verified {} doubled values", output.len());
    Ok(())
}
