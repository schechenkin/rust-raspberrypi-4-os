// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2023 Andre Richter <andre.o.richter@gmail.com>

//! Printing.

use crate::console;
use core::fmt;

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    console::console().write_fmt(args).unwrap();
}

/// Prints without a newline.
///
/// Copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}

/// Prints with a newline.
///
/// Copy from <https://os.phil-opp.com/printing-to-screen/#a-println-macro>
#[macro_export]
macro_rules! println {
    ($fmt:expr) => ($crate::print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::print!(concat!($fmt, "\n"), $($arg)*));
}

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------
/// Prints an info, with a newline.
#[macro_export]
macro_rules! info {
    ($string:expr) => {{
        let timestamp = $crate::time::time_manager().uptime();

        $crate::println!(
            concat!("[Info  {:>3}.{:06}] ", $string),
            timestamp.as_secs(),
            timestamp.subsec_micros()
        );
    }};
    ($($arg:tt)*) => {{
        let timestamp = $crate::time::time_manager().uptime();
        $crate::print!("[Info  {:>3}.{:06}] ", timestamp.as_secs(),timestamp.subsec_micros());
        $crate::println!($($arg)*);
    }};
}

/// Prints a warning, with a newline.
#[macro_export]
macro_rules! warn {
    ($string:expr) => {{
        let timestamp = $crate::time::time_manager().uptime();

        $crate::println!(
            concat!("[Warn  {:>3}.{:06}] ", $string),
            timestamp.as_secs(),
            timestamp.subsec_micros()
        );
    }};
    ($($arg:tt)*) => {{
        let timestamp = $crate::time::time_manager().uptime();
        $crate::print!("[Warn  {:>3}.{:06}] ", timestamp.as_secs(),timestamp.subsec_micros());
        $crate::println!($($arg)*);
    }};
}
