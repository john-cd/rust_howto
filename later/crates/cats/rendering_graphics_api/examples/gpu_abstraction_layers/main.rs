#[cfg(feature = "wgpu")]
mod wgpu;

fn main() {
    #[cfg(feature = "wgpu")]
    {
        let _ = wgpu::run();
    }
}
