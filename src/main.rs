#![no_std]
#![no_main]

mod boot;
mod bsp;
mod console;
mod cpu;
mod driver;
mod panic_wait;
mod print;
mod synchronization;
mod time;

use ansi_rgb::{Foreground, red};

/// Early init code.
///
/// # Safety
///
/// - Only a single core must be active and running this function.
/// - The init calls in this function must appear in the correct order.
unsafe fn kernel_init() -> ! {
    // Initialize the BSP driver subsystem.
    if let Err(x) = unsafe { bsp::driver::init() } {
        panic!("Error initializing BSP driver subsystem: {}", x);
    }

    // Initialize all device drivers.
    unsafe { driver::driver_manager().init_drivers() };
    // println! is usable from here on.

    // Transition from unsafe to safe.
    kernel_main()
}

/// The main function running after the early init.
fn kernel_main() -> ! {
    use core::time::Duration;

    //println!("{}", "Hello, world!".fg(red()));

    info!("Test info!");

    info!(
        "{} version {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    info!("Booting on: {}", bsp::board_name());

    info!(
        "Architectural timer resolution: {} ns",
        time::time_manager().resolution().as_nanos()
    );

    info!("Drivers loaded:");
    driver::driver_manager().enumerate();

    // Test a failing timer case.
    time::time_manager().spin_for(Duration::from_nanos(1));

    loop {
        info!("Spinning for 1 second");
        time::time_manager().spin_for(Duration::from_secs(1));
    }
}
