#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod arch_boot;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
