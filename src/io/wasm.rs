pub fn print(record: &log::Record) {
    use web_sys::console;

    #[cfg(feature="timestamp")]
    let string = format!("{:<5} [{}] - {} at {}:{}", record.level(), super::get_date(), record.args(), record.file().unwrap_or("UNKNOWN"), record.line().unwrap_or(0));

    #[cfg(not(feature="timestamp"))]
    let string = format!("{:<5} - {} at {}:{}", record.level(), record.args(), record.file().unwrap_or("UNKNOWN"), record.line().unwrap_or(0));

    match record.level() {
        log::Level::Trace => console::debug_1(&string.into()),
        log::Level::Debug => console::log_1(&string.into()),
        log::Level::Info => console::info_1(&string.into()),
        log::Level::Warn => console::warn_1(&string.into()),
        log::Level::Error => console::error_1(&string.into()),
    }
}
