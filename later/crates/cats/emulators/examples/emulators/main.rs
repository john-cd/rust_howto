mod emulator;

fn main() {
    emulator::run().expect("PolkaVM example failed");
}
