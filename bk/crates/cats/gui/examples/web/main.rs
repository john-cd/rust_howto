mod dioxus;
mod tauri;

fn main() {
    // [ P1](https://github.com/john-cd/rust_howto/issues/1055)
    dioxus::main();
    tauri::main();
}
