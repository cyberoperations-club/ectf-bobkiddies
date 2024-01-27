#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[link(name = "../component")]
extern "C" {
    pub fn i2c_simple_isr();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
