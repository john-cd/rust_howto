mod bindgen;
mod cbindgen;

fn main() {
    bindgen::run();
    cbindgen::run();
}
