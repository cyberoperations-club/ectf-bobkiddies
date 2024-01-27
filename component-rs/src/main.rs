#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[link(name = "../component")]
extern "C" {
    #[allow(dead_code)] // not finished
    fn i2c_simple_isr();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
