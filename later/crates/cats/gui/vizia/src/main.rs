mod signal_counter;

fn main() -> impl std::process::Termination {
    signal_counter::run()
}
