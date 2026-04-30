mod winapi;
mod windows;

fn main() {
    #[cfg(target_os = "windows")]
    windows::run();
}
