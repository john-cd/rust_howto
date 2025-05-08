// ANCHOR: example
#![cfg(target_os = "windows")]
//! Example of a basic Windows application using the `winapi` crate.

use std::ptr::null_mut;

use winapi::shared::minwindef::LPARAM;
use winapi::shared::minwindef::LRESULT;
use winapi::shared::minwindef::UINT;
use winapi::shared::minwindef::WPARAM;
// use winapi::shared::windef::HBRUSH;
// use winapi::shared::windef::HINSTANCE;
use winapi::shared::windef::HWND;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winuser::CS_HREDRAW;
use winapi::um::winuser::CS_VREDRAW;
use winapi::um::winuser::CreateWindowExW;
use winapi::um::winuser::DefWindowProcW;
use winapi::um::winuser::DispatchMessageW;
use winapi::um::winuser::GetMessageW;
// use winapi::um::winuser::MB_ICONINFORMATION;
// use winapi::um::winuser::MB_OK;
use winapi::um::winuser::MSG;
// use winapi::um::winuser::MessageBoxW;
use winapi::um::winuser::RegisterClassW;
use winapi::um::winuser::SW_SHOW;
use winapi::um::winuser::ShowWindow;
use winapi::um::winuser::TranslateMessage;
use winapi::um::winuser::WNDCLASSW;

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        // Handle the WM_DESTROY message, which is sent when the window is being
        // destroyed. This is where we post a quit message to the
        // message queue, which will eventually cause the `GetMessageW`
        // function to return 0, ending the message loop.
        //
        // Note that we use unsafe here, because we're calling a raw Windows API
        // function.
        //
        // The return value of 0 indicates that the message was processed.
        // If we returned `DefWindowProcW`, the default processing would be
        // performed.
        winapi::um::winuser::WM_DESTROY => {
            unsafe {
                winapi::um::winuser::PostQuitMessage(0);
            }
            0
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

/// Converts a Rust string slice to a null-terminated wide string (`Vec<u16>`).
///
/// This is necessary because many Windows API functions expect wide strings
/// (UTF-16) instead of UTF-8 strings.
fn to_wstring(str: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    OsStr::new(str).encode_wide().chain(Some(0)).collect()
}

/// Basic Windows application with a message box.
fn main() {
    unsafe {
        let h_instance = GetModuleHandleW(null_mut());
        // Define the window class name.
        let class_name = to_wstring("window");
        // Define the window class structure.
        let wnd = WNDCLASSW {
            // Style of the window class.
            style: CS_HREDRAW | CS_VREDRAW,
            // Pointer to the window procedure function.
            // This function will handle messages sent to the window.
            lpfnWndProc: Some(window_proc),
            hInstance: h_instance,
            lpszClassName: class_name.as_ptr(),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hIcon: null_mut(),
            hCursor: null_mut(),
            hbrBackground: null_mut(),
            lpszMenuName: null_mut(),
        };

        // Register the window class.
        RegisterClassW(&wnd);

        let hwnd = CreateWindowExW(
            0,
            class_name.as_ptr(),
            to_wstring("Hello, winapi!").as_ptr(),
            winapi::um::winuser::WS_OVERLAPPEDWINDOW,
            winapi::um::winuser::CW_USEDEFAULT,
            winapi::um::winuser::CW_USEDEFAULT,
            winapi::um::winuser::CW_USEDEFAULT,
            winapi::um::winuser::CW_USEDEFAULT,
            null_mut(),
            null_mut(),
            h_instance,
            null_mut(),
        );

        // Show the window.
        ShowWindow(hwnd, SW_SHOW);

        let mut msg: MSG = std::mem::zeroed();

        // GetMessageW retrieves messages from the message queue.
        // It returns 0 when a WM_QUIT message is received, which indicates that
        // the application should exit. It returns -1 on error.
        while GetMessageW(&mut msg, null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}
// ANCHOR_END: example

#[test]
#[ignore = "Needs review"]
fn test() {
    main();
}
// [review / test](https://github.com/john-cd/rust_howto/issues/822)
