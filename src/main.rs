#![no_std]
#![no_main]

mod boot;
mod bsp;
mod console;
mod cpu;
mod panic_wait;
mod print;

/// Early init code.
///
/// # Safety
///
/// - Only a single core must be active and running this function.
unsafe fn kernel_init() -> ! {
    print!("Hello from Rust!");

    panic!("Stopping here.")
}
