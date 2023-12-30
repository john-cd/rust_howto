fn main() {
    tracing_subscriber::fmt::init(); // filter events at runtime using environment variables: RUST_LOG=debug,my_crate=trace
}
