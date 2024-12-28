// `OpenCV` requires `clang`, which is not installed by default on Windows
#[cfg(target_os = "linux")]
mod opencv;
