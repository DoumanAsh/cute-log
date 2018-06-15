//!This crate provides simple and cute logger.
//!
//!## Feautres
//!
//!- `timestamp` - Enables timestamps in logs by means of `chrono`. Enabled by default
//!- `color` - Enables coloring of log level. Enabled by default.
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
//!
//!## Log level control
//!
//!The logger is made without any builtin filters.
//!
//!You can either control logs through compile time features of `log` crate.
//!Or use `set_max_level`.

extern crate log;

mod io;

///Simple Logger implementation
///
///It provides logger without filtering with following format:
///`<level> [<date and time>] <file>:<line> - <args>`
///
///Timestamp can be turned off by disabling default features
pub struct Logger;

impl Logger {
    ///Logger printer.
    pub fn print(record: &log::Record) {
        io::print(record);
    }

    ///Sets `log` max level
    pub fn set_max_level(level: log::LevelFilter) {
        log::set_max_level(level);
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
