#[cfg(not(windows))] // TODO review Windows support for Java FFI examples
mod jni;

fn main() {
    #[cfg(not(windows))]
    jni::run();
}
