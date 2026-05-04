use anyhow::Result;

mod rust_gpu;

fn main() -> Result<()> {
    rust_gpu::run();
    Ok(())
}
