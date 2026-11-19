// TODO Reviewed ??-??-2026.

// ANCHOR: example
use std::error::Error;
#[cfg(any(target_os = "linux", target_os = "macos"))]
use std::ffi::c_char;

use libloading::Library;

/// Calls a known function from the operating system's C library.
fn query_system_value() -> Result<usize, Box<dyn Error>> {
    #[cfg(target_os = "windows")]
    {
        // SAFETY: kernel32.dll is supplied by Windows and is a trusted system library.
        let library = unsafe { Library::new("kernel32.dll")? };
        // SAFETY: GetCurrentProcessId has this signature and needs no arguments.
        let process_id =
            unsafe { library.get::<unsafe extern "system" fn() -> u32>(b"GetCurrentProcessId\0")? };
        // Keep the library alive until after calling the function pointer.
        Ok(unsafe { process_id() as usize })
    }

    #[cfg(target_os = "linux")]
    {
        // SAFETY: libc.so.6 is the system C library on GNU/Linux.
        let library = unsafe { Library::new("libc.so.6")? };
        // SAFETY: strlen accepts a valid NUL-terminated C string and returns its byte length.
        let strlen =
            unsafe { library.get::<unsafe extern "C" fn(*const c_char) -> usize>(b"strlen\0")? };
        let input = c"Rust FFI";
        // SAFETY: the input is NUL-terminated and lives through the call.
        Ok(unsafe { strlen(input.as_ptr()) })
    }

    #[cfg(target_os = "macos")]
    {
        // SAFETY: libSystem is supplied by macOS and is a trusted system library.
        let library = unsafe { Library::new("/usr/lib/libSystem.B.dylib")? };
        // SAFETY: strlen accepts a valid NUL-terminated C string and returns its byte length.
        let strlen =
            unsafe { library.get::<unsafe extern "C" fn(*const c_char) -> usize>(b"strlen\0")? };
        let input = c"Rust FFI";
        // SAFETY: the input is NUL-terminated and lives through the call.
        Ok(unsafe { strlen(input.as_ptr()) })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("system lookup returned {}", query_system_value()?);
    Ok(())
}
// ANCHOR_END: example

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
compile_error!("this example supports Windows, GNU/Linux, and macOS");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dynamically_loaded_system_function_returns_a_value() -> Result<(), Box<dyn Error>> {
        let value = query_system_value()?;
        #[cfg(target_os = "windows")]
        assert!(value > 0);
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        assert_eq!(value, "Rust FFI".len());
        Ok(())
    }
}
