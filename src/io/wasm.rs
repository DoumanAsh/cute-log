use web_log::{ConsoleType, Console};
use core::fmt::Write;

impl crate::Logger {
    #[inline(always)]
    ///Prints plain text with as `Info`
    pub fn print_fmt(args: core::fmt::Arguments<'_>) {
        let mut console = Console::new(ConsoleType::Info);

        let _ = write!(console, "{}", args);
    }

    #[inline]
    ///Logger printer.
    pub fn print(record: &log::Record) {
        let mut console = match record.level() {
            log::Level::Trace => Console::new(ConsoleType::Debug),
            log::Level::Debug => Console::new(ConsoleType::Debug),
            log::Level::Info => Console::new(ConsoleType::Info),
            log::Level::Warn => Console::new(ConsoleType::Warn),
            log::Level::Error => Console::new(ConsoleType::Error),
        };

        let _ = write!(console, "{:<5} {{{}:{}}} - {}", record.level(), record.file().unwrap_or("UNKNOWN"), record.line().unwrap_or(0), record.args());
    }
}
