#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

/// Error handler
/// We will never call this due to our cargo configuration
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}
