//!This crate provides simple and cute logger.
//!
//!## Feautres
//!
//!- `timestamp` - Enables timestamps in logs by means of `chrono`. Enabled by default
//!
//!## Usage
//!
//!```rust
//!#[macro_use]
//!extern crate log;
//!extern crate cute_log;
//!
//!fn main() {
//!    cute_log::init();
//!    info!("it works!");
//!}
//!```

extern crate log;
#[cfg(feature="timestamp")]
extern crate chrono;

use std::fmt;

///Simple Logger implementation
///
///It provides logger without filtering with following format:
///`<level> [<date and time>] <file>:<line> - <args>`
///
///Timestamp can be turned off by disabling default features
pub struct Logger;

impl Logger {
    #[cfg(feature="timestamp")]
    fn get_date() -> impl fmt::Display {
        chrono::offset::Local::now().format("%F %H:%M:%S%.3f %z")
    }

    ///Logger printer.
    pub fn print(record: &log::Record) {
        #[cfg(feature="timestamp")]
        {
            println!("{:<5} [{}] {}:{} - {}",
                     record.level(),
                     Self::get_date(),
                     record.file().unwrap_or("UNKNOWN"), record.line().unwrap_or(0),
                     record.args());

        }

        #[cfg(not(feature="timestamp"))]
        {
            println!("{:<5} {}:{} - {}",
                     record.level(),
                     record.file().unwrap_or("UNKNOWN"), record.line().unwrap_or(0),
                     record.args());
        }
    }
}

impl log::Log for Logger {
    #[inline]
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    #[inline]
    fn log(&self, record: &log::Record) {
        Self::print(record);
    }

    #[inline]
    fn flush(&self) {
    }
}

#[inline]
///Sets global logger with log level Trace
pub fn init() -> Result<(), log::SetLoggerError> {
    init_with_max_level(log::LevelFilter::Trace)
}

///Sets logger with max log level.
pub fn init_with_max_level(level: log::LevelFilter) -> Result<(), log::SetLoggerError> {
    static INSTANCE: Logger = Logger;
    log::set_max_level(level);
    log::set_logger(&INSTANCE)
}
