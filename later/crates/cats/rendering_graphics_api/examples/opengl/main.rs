//! Demonstrates loading and calling the cross-platform `glow` bindings.

// ANCHOR: example
use std::ffi::CStr;
use std::ffi::CString;
use std::ffi::c_void;
use std::ptr;

use glow::HasContext;

/// Converts a function name to a C string for a graphics API loader.
fn c_symbol(name: &str) -> Option<CString> {
    CString::new(name).ok()
}

/// Creates a `glow` context using function pointers from a current GL context.
///
/// # Safety
///
/// The caller must make the intended OpenGL context current before loading its
/// function pointers and keep that context valid while using the returned API.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "The host must supply a current context; this example has no window system."
    )
)]
unsafe fn load_context(
    mut get_proc_address: impl FnMut(&CStr) -> *const c_void,
) -> glow::Context {
    // SAFETY: the caller guarantees the platform context is current and valid.
    unsafe {
        glow::Context::from_loader_function(|name| {
            let Some(name) = c_symbol(name) else {
                return ptr::null();
            };
            get_proc_address(&name)
        })
    }
}

/// Clears the current framebuffer to a dark blue color.
///
/// # Safety
///
/// `gl` must belong to the current context, which must remain current for the
/// duration of both OpenGL calls.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "The host must supply a current context; this example has no window system."
    )
)]
unsafe fn clear_background(gl: &glow::Context) {
    // SAFETY: the caller guarantees `gl` is for the current context.
    unsafe {
        gl.clear_color(0.08, 0.12, 0.2, 1.0);
        gl.clear(glow::COLOR_BUFFER_BIT);
    }
}

/// Describes the context setup required before using the helpers above.
fn main() {
    println!(
        "A GL loader resolves symbols such as {:?}.",
        c_symbol("glClear")
    );
    println!(
        "Create and make an OpenGL context current before calling the glow helpers."
    );
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use std::ffi::c_void;
    use std::ptr;

    use super::c_symbol;

    /// Returns a mock OpenGL version and extension string.
    unsafe extern "system" fn stub_get_string(_: u32) -> *const u8 {
        c"2.1 Mock".as_ptr().cast::<u8>()
    }

    /// Mock OpenGL clear-color function used to load a test context.
    unsafe extern "system" fn stub_clear_color(_: f32, _: f32, _: f32, _: f32) {
    }

    /// Mock OpenGL clear function used to load a test context.
    unsafe extern "system" fn stub_clear(_: u32) {}

    /// Runs the display-free entry point so Cargo tests do not require a
    /// window.
    #[test]
    fn test_main_entry_point() {
        super::main();
    }

    /// Converts a valid OpenGL symbol name and rejects an embedded NUL byte.
    #[test]
    fn test_c_symbol_conversion() {
        let symbol =
            c_symbol("glClear").expect("valid symbol names have no NUL bytes");
        assert_eq!(symbol.as_c_str().to_bytes(), b"glClear");
        assert!(c_symbol("gl\0Clear").is_none());
    }

    /// Loads the GL helpers through stubs and clears without a graphics device.
    #[test]
    fn test_context_loader_and_clear_helpers() {
        let gl = unsafe {
            super::load_context(|name| match name.to_bytes() {
                b"glGetString" => {
                    (stub_get_string as *const ()).cast::<c_void>()
                }
                b"glClearColor" => {
                    (stub_clear_color as *const ()).cast::<c_void>()
                }
                b"glClear" => (stub_clear as *const ()).cast::<c_void>(),
                _ => ptr::null(),
            })
        };

        // SAFETY: the test loader gives these functions to the matching GL
        // calls.
        unsafe { super::clear_background(&gl) };
    }
}
