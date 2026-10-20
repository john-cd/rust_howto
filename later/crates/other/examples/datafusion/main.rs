mod datafusion;

fn main() {
    #[cfg(feature = "datafusion")]
    datafusion::run();
}
