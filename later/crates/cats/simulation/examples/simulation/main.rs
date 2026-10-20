#[cfg(not(windows))] // TODO review Windows support for simulation examples
mod simulation1;

fn main() {
    #[cfg(not(windows))]
    simulation1::run();
}
