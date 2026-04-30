mod wasmer;
#[cfg(feature = "wasmtime")]
mod wasmtime;

fn main() {
    wasmer::run();
    wasmtime::run();
}
