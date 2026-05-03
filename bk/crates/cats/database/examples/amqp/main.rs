#[cfg(feature = "lapin")]
mod lapin;

fn main() {
    #[cfg(feature = "lapin")]
    {
        lapin::run();
    }
}
