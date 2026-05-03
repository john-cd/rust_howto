#[cfg(not(windows))]
mod rapier2d;

fn main() {
    #[cfg(not(windows))]
    // TODO - add a Windows-compatible physics engine example
    rapier2d::run();
}
