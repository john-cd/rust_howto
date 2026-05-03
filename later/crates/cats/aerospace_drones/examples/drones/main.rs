mod drones1;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    drones1::run().map_err(|e| e.into())
}
