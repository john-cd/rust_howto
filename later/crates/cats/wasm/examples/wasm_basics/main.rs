mod inspect_wasm_module;

fn main() -> impl std::process::Termination {
    inspect_wasm_module::run()
}
