#![allow(dead_code)]
// ANCHOR: example
//! A macOS-specific example using the `cocoa` crate to call native macOS APIs.
//!
//! On macOS this example queries the current user name and operating system
//! version using `NSProcessInfo`.

#[cfg(target_os = "macos")]
mod macos_impl {
    use cocoa::base::id;
    use cocoa::base::nil;
    use cocoa::foundation::NSAutoreleasePool;
    use cocoa::foundation::NSString;
    use objc::class;
    use objc::msg_send;
    use objc::sel;
    use objc::sel_impl;

    fn nsstring_to_string(nsstring: id) -> String {
        unsafe {
            if nsstring == nil {
                return String::new();
            }
            let bytes: *const std::os::raw::c_char =
                msg_send![nsstring, UTF8String];
            std::ffi::CStr::from_ptr(bytes)
                .to_string_lossy()
                .into_owned()
        }
    }

    pub fn run() {
        unsafe {
            let _pool = NSAutoreleasePool::new(nil);
            let process_info: id =
                msg_send![class!(NSProcessInfo), processInfo];
            let os_version: id =
                msg_send![process_info, operatingSystemVersionString];
            let user_name: id = msg_send![process_info, userName];

            println!("macOS user: {}", nsstring_to_string(user_name));
            println!("Operating system: {}", nsstring_to_string(os_version));
        }
    }
}

#[cfg(not(target_os = "macos"))]
pub fn main() {
    println!("This example uses macOS APIs and is only supported on macOS.");
}

#[cfg(target_os = "macos")]
pub fn main() {
    macos_impl::run();
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        main();
    }
}
