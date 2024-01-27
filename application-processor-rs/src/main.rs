#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[link(name = "../application_processor")]
extern "C" {
    pub fn i2c_simple_controller_init();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
