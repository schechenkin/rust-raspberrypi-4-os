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

/// Early init code.
///
/// # Safety
///
/// - Only a single core must be active and running this function.
/// - The init calls in this function must appear in the correct order:
///     - MMU + Data caching must be activated at the earliest. Without it, any atomic operations,
///       e.g. the yet-to-be-introduced spinlocks in the device drivers (which currently employ
///       NullLocks instead of spinlocks), will fail to work (properly) on the RPi SoCs.
unsafe fn kernel_init() -> ! {
    use memory::mmu::interface::MMU;

    unsafe { exception::handling_init() };

    if let Err(string) = unsafe { memory::mmu::mmu().enable_mmu_and_caching() } {
        panic!("MMU: {}", string);
    }

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

    // Cause an exception by accessing a virtual address for which no translation was set up. This
    // code accesses the address 8 GiB, which is outside the mapped address space.
    //
    // For demo purposes, the exception handler will catch the faulting 8 GiB address and allow
    // execution to continue.
    info!("");
    info!("Trying to read from address 8 GiB...");
    let mut big_addr: u64 = 8 * 1024 * 1024 * 1024;
    unsafe { core::ptr::read_volatile(big_addr as *mut u64) };

    info!("************************************************");
    info!("Whoa! We recovered from a synchronous exception!");
    info!("************************************************");
    info!("");
    info!("Let's try again");

    // Now use address 9 GiB. The exception handler won't forgive us this time.
    info!("Trying to read from address 9 GiB...");
    big_addr = 9 * 1024 * 1024 * 1024;
    unsafe { core::ptr::read_volatile(big_addr as *mut u64) };

    // Will never reach here in this tutorial.
    info!("Echoing input now");

    // Discard any spurious received characters before going into echo mode.
    console().clear_rx();
    loop {
        let c = console().read_char();
        console().write_char(c);
    }
}
