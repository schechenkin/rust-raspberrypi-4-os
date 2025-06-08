#![no_std]
#![no_main]

mod boot;
mod bsp;
mod common;
mod console;
mod cpu;
mod driver;
mod exception;
mod memory;
mod panic_wait;
mod print;
mod synchronization;
mod time;

use crate::console::interface::Write;

/// Early init code.
///
/// # Safety
///
/// - Only a single core must be active and running this function.
/// - The init calls in this function must appear in the correct order.
unsafe fn kernel_init() -> ! {
    use memory::mmu::interface::MMU;

    // Initialize the BSP driver subsystem.
    if let Err(x) = unsafe { bsp::driver::init() } {
        panic!("Error initializing BSP driver subsystem: {}", x);
    }

    // Initialize all device drivers.
    unsafe { driver::driver_manager().init_drivers() };
    // println! is usable from here on.

    if let Err(string) = unsafe { memory::mmu::mmu().enable_mmu_and_caching() } {
        panic!("MMU: {}", string);
    }

    // Transition from unsafe to safe.
    kernel_main()
}

/// The main function running after the early init.
fn kernel_main() -> ! {
    use console::console;
    use core::time::Duration;

    info!(
        "{} version {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    info!("Booting on: {}", bsp::board_name());

    info!("MMU online. Special regions:");
    bsp::memory::mmu::virt_mem_layout().print_layout();

    let (_, privilege_level) = exception::current_privilege_level();
    info!("Current privilege level: {}", privilege_level);

    info!("Exception handling state:");
    exception::asynchronous::print_state();

    info!(
        "Architectural timer resolution: {} ns",
        time::time_manager().resolution().as_nanos()
    );

    info!("Drivers loaded:");
    driver::driver_manager().enumerate();

    info!("Timer test, spinning for 1 second");
    time::time_manager().spin_for(Duration::from_secs(1));

    let remapped_uart = unsafe { bsp::device_driver::PL011Uart::new(0x1FFF_1000) };
    writeln!(
        remapped_uart,
        "[     !!!    ] Writing through the remapped UART at 0x1FFF_1000"
    )
    .unwrap();

    info!("Echoing input now");

    // Discard any spurious received characters before going into echo mode.
    console().clear_rx();
    loop {
        let c = console().read_char();
        console().write_char(c);
    }
}
