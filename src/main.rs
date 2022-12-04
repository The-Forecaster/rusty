#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// Error handler
/// We will never call this due to our cargo configuration
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8; // Buffer where we print the text

    for (i, &byte) in HELLO.iter().enumerate() { // Make this into a tuple list of (index, item)
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte; // put the byte into the offset
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // reset the offset
        }
    }

    loop {} // Loop forever so the PC doesn't shut off
}
