mod blink_output_pin;

fn main() -> impl std::process::Termination {
    blink_output_pin::run()
}
