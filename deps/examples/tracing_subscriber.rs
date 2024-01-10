fn main() {
    // Filter events at runtime using the value
    // of the RUST_LOG environment variable:
    // for example, RUST_LOG=debug,my_crate=trace
    tracing_subscriber::fmt::init();
}
