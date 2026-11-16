mod i2c_temperature;

fn main() -> impl std::process::Termination {
    i2c_temperature::run()
}
