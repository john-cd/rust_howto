mod embassy;

fn main() -> impl std::process::Termination {
    embassy::run()
}
