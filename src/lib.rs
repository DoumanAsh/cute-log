//!This crate provides simple and cute logger.
//!
//!## Features
//!
//!- `timestamp` - Enables timestamps in logs by means of `chrono`. Enabled by default
//!- `std` - Enables use of `std` feature to provide `RUST_LOG` handling.
//!
//!## Usage
//!
//!```rust
//!const LOGGER: cute_log::Logger = cute_log::Logger::new();
//!LOGGER.set_max_level(cute_log::log::LevelFilter::Info);
//!let _ = LOGGER.set_logger();
//!log::info!("it works!");
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

#![no_std]

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

    #[cfg(feature = "std")]
    ///Sets `log` max level from `RUST_LOG` or if not available, use provided default level.
    ///
    ///Requires `std` feature
    pub fn set_log_env_or(&self, default_level: log::LevelFilter) {
        extern crate std;

        fn parse_rust_log(log: &str, default_level: log::LevelFilter) -> log::LevelFilter {
            if log.eq_ignore_ascii_case("off") {
                log::LevelFilter::Off
            } else if log.eq_ignore_ascii_case("error") {
                log::LevelFilter::Error
            } else if log.eq_ignore_ascii_case("warn") {
                log::LevelFilter::Warn
            } else if log.eq_ignore_ascii_case("info") {
                log::LevelFilter::Info
            } else if log.eq_ignore_ascii_case("debug") {
                log::LevelFilter::Debug
            } else if log.eq_ignore_ascii_case("trace") {
                log::LevelFilter::Trace
            } else {
                default_level
            }
        }

        match std::env::var("RUST_LOG") {
            Ok(log) => self.set_max_level(parse_rust_log(&log, default_level)),
            Err(_) => self.set_max_level(default_level),
        }
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
