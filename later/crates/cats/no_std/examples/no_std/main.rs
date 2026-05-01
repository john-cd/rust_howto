#![no_std]
#![no_main]

mod no_std1;
use core::panic::PanicInfo;

#[export_name = "main"]
pub extern "C" fn no_std_main() -> ! {
    no_std1::run();
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
