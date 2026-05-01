#[cfg(feature = "lapin")]
mod lapin;

fn main() {
    #[cfg(feature = "lapin")]
    {
        let _ = lapin::run();
    }
}
