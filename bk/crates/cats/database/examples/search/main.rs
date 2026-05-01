#[cfg(feature = "elasticsearch")]
mod elasticsearch;

fn main() {
    #[cfg(feature = "elasticsearch")]
    {
        let _ = elasticsearch::run();
    }
}
