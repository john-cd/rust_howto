// TODO Reviewed ??-??-2026.

mod external_dependency;

fn main() -> impl std::process::Termination {
    external_dependency::run()
}
