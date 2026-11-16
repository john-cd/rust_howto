mod fixed_priority_schedule;

fn main() -> impl std::process::Termination {
    fixed_priority_schedule::run()
}
