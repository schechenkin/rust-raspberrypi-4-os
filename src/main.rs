#![no_std]
#![no_main]

mod boot;
mod bsp;
mod cpu;
use core::panic::PanicInfo;

/// Early init code.
///
/// # Safety
///
/// - Only a single core must be active and running this function.
unsafe fn kernel_init() -> ! {
    panic!()
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    cpu::wait_forever()
}
