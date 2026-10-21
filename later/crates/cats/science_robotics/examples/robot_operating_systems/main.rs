// TODO Reviewed ??-??-2026
#[cfg(all(feature = "rosrust", not(windows)))]
mod robotics;

fn main() {
    #[cfg(all(feature = "rosrust", not(windows)))]
    robotics::run();
}
