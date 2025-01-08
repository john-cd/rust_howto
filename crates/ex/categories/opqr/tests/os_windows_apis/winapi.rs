// ANCHOR: example
use std::ptr::null_mut;

use winapi::shared::minwindef::LPARAM;
use winapi::shared::minwindef::LRESULT;
use winapi::shared::minwindef::UINT;
use winapi::shared::minwindef::WPARAM;
//use winapi::shared::windef::HBRUSH;
//use winapi::shared::windef::HINSTANCE;
use winapi::shared::windef::HWND;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winuser::CS_HREDRAW;
use winapi::um::winuser::CS_VREDRAW;
use winapi::um::winuser::CreateWindowExW;
use winapi::um::winuser::DefWindowProcW;
use winapi::um::winuser::DispatchMessageW;
use winapi::um::winuser::GetMessageW;
//use winapi::um::winuser::MB_ICONINFORMATION;
//use winapi::um::winuser::MB_OK;
use winapi::um::winuser::MSG;
//use winapi::um::winuser::MessageBoxW;
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
        winapi::um::winuser::WM_DESTROY => {
            unsafe {
                winapi::um::winuser::PostQuitMessage(0);
            }
            return 0;
        }
        _ => unsafe { return DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

fn to_wstring(str: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    OsStr::new(str)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect()
}

// Basic Windows application with a message box.
fn main() {
    unsafe {
        let h_instance = GetModuleHandleW(null_mut());

        let class_name = to_wstring("window");
        let wnd = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
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

        ShowWindow(hwnd, SW_SHOW);

        let mut msg: MSG = std::mem::zeroed();

        while GetMessageW(&mut msg, null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}
// ANCHOR_END: example

#[test]
#[ignore = "WIP"]
fn test() {
    main();
}
// [P2](https://github.com/john-cd/rust_howto/issues/822)
