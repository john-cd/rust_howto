mod exponential_backoff;

fn main() -> impl std::process::Termination {
    exponential_backoff::run()
}
