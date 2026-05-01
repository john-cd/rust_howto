#[cfg(feature = "elasticsearch")]
mod elasticsearch;

fn main() {
    #[cfg(feature = "elasticsearch")]
    {
        elasticsearch::run();
    }
}
