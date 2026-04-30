mod napi;
mod neon;

fn main() {
    napi::run();
    neon::run();
}
