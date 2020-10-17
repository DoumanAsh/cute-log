//!This crate provides simple and cute logger.
//!
//!## Features
//!
//!- `timestamp` - Enables timestamps in logs by means of `chrono`. Enabled by default
//!
//!## Usage
//!
//!```rust
//!fn main() {
//!    const LOGGER: cute_log::Logger = cute_log::Logger::new();
//!    LOGGER.set_max_level(cute_log::log::LevelFilter::Info);
//!    let _ = LOGGER.set_logger();
//!    log::info!("it works!");
//!}
//!```
//!
//!## Log level control
//!
//!The logger is made without any builtin filters.
//!
//!You can either control logs through compile time features of `log` crate.
//!Or use `set_max_level` from the `log` crate.
//!
//!## Supported platforms
//!
//!- Android - via NDK logging library, therefore it must be linked.
//!- Wasm - via web console API.
//!- Any other platform with `std` available.

#![cfg_attr(any(target_os = "android", target_arch = "wasm32", target_os = "unknown"), no_std)]

pub extern crate log;

mod io;

///Simple Logger implementation
///
///It provides logger without filtering with following format:
///`<level> [<date and time>] {<file>:<line>} - <args>`
///
///Timestamp is only used on `std` environment aside from Android.
pub struct Logger;

impl Logger {
    #[inline]
    ///Creates new instance.
    pub const fn new() -> Self {
        Self
    }

    #[inline]
    ///Sets `log` max level, controlling which level to print at most.
    ///
    ///By default it is off.
    pub fn set_max_level(&self, level: log::LevelFilter) {
        log::set_max_level(level);
    }

    #[inline]
    ///Initialize self as `log` global logger.
    pub fn set_logger(&'static self) -> Result<(), log::SetLoggerError> {
        log::set_logger(self)
    }

    #[inline]
    ///Initialize self as `log` global logger in debug mode only.
    pub fn set_logger_debug(&'static self) -> Result<(), log::SetLoggerError> {
        #[cfg(debug_assertions)]
        {
            log::set_logger(self)
        }
        #[cfg(not(debug_assertions))]
        {
            Ok(())
        }
    }
}

impl log::Log for Logger {
    #[inline(always)]
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    #[inline(always)]
    fn log(&self, record: &log::Record) {
        Self::print(record);
    }

    #[inline(always)]
    fn flush(&self) {
    }
}
