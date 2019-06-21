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
//!fn main() {
//!    cute_log::init();
//!    log::info!("it works!");
//!}
//!```
//!
//!## Log level control
//!
//!The logger is made without any builtin filters.
//!
//!You can either control logs through compile time features of `log` crate.
//!Or use `set_max_level`.

mod io;

///Simple Logger implementation
///
///It provides logger without filtering with following format:
///`<level> [<date and time>] - <args> at <file>:<line>`
///
///Timestamp can be turned off by disabling default features
pub struct Logger;

impl Logger {
    #[inline]
    ///Logger printer.
    pub fn print(record: &log::Record) {
        io::print(record);
    }

    #[inline]
    ///Sets `log` max level
    pub fn set_max_level(level: log::LevelFilter) {
        log::set_max_level(level);
    }
}

impl log::Log for Logger {
    #[inline(always)]
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    #[inline]
    fn log(&self, record: &log::Record) {
        Self::print(record);
    }

    #[inline(always)]
    fn flush(&self) {
    }
}

#[inline(always)]
///Sets global logger as [init](fn.init.html) only in debug mode.
pub fn debug_init() -> Result<(), log::SetLoggerError> {
    #[cfg(debug_assertions)]
    {
        init()
    }
    #[cfg(not(debug_assertions))]
    {
        Ok(())
    }
}

#[inline]
///Sets global logger with log level Trace
pub fn init() -> Result<(), log::SetLoggerError> {
    init_with_max_level(log::LevelFilter::Trace)
}

#[inline(always)]
///Sets global logger as [init_with_max_level](fn.init_with_max_level.html) only in debug mode.
pub fn debug_init_with_max_level(_level: log::LevelFilter) -> Result<(), log::SetLoggerError> {
    #[cfg(debug_assertions)]
    {
        init_with_max_level(_level)
    }
    #[cfg(not(debug_assertions))]
    {
        Ok(())
    }
}

///Sets logger with max log level.
pub fn init_with_max_level(level: log::LevelFilter) -> Result<(), log::SetLoggerError> {
    static INSTANCE: Logger = Logger;
    log::set_max_level(level);
    log::set_logger(&INSTANCE)
}
