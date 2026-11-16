// TODO Reviewed ??-??-2026.

mod signal_counter;

fn main() -> impl std::process::Termination {
    signal_counter::run()
}
