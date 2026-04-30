#[cfg(feature = "wgpu")]
mod wgpu;

fn main() {
    #[cfg(feature = "wgpu")]
    wgpu::run();
}
