mod scheduled_command;

fn main() -> impl std::process::Termination {
    scheduled_command::run()
}
