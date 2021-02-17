use androidy_log::{Writer, LogPriority};

use core::fmt::Write;

impl crate::Logger {
    #[inline(always)]
    ///Prints plain text with `INFO` priority
    pub fn print_fmt(args: core::fmt::Arguments<'_>) {
        let mut writer = Writer::new_default(LogPriority::INFO);
        let _ = write!(writer, "{}", args);
    }

    #[inline]
    ///Logger printer.
    pub fn print(record: &log::Record) {
        let prio = match record.level() {
            log::Level::Warn => LogPriority::WARN,
            log::Level::Info => LogPriority::INFO,
            log::Level::Debug => LogPriority::DEBUG,
            log::Level::Error => LogPriority::ERROR,
            log::Level::Trace => LogPriority::VERBOSE,
        };

        let mut writer = match record.module_path() {
            Some(module) => Writer::new(module, prio),
            None => Writer::new_default(prio),
        };

        let _ = write!(writer, "{{{}:{}}} - {}", record.file().unwrap_or("UNKNOWN"), record.line().unwrap_or(0), record.args());
    }
}
