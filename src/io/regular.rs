extern crate std;

#[cfg(feature="timestamp")]
#[inline(always)]
fn get_date() -> impl core::fmt::Display {
    struct TimeDate(time::OffsetDateTime);

    impl core::fmt::Display for TimeDate {
        #[inline(always)]
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "{}-{:02}-{:02} {:02}:{:02}:{:02}", self.0.year(), self.0.month() as u8, self.0.day(), self.0.hour(), self.0.minute(), self.0.second())
        }
    }

    TimeDate(std::time::SystemTime::now().into())
}

impl crate::Logger {
    #[inline(always)]
    ///Prints to `stdout` as it is
    pub fn print_fmt(args: core::fmt::Arguments<'_>) {
        std::println!("{}", args);
    }

    #[inline]
    ///Logger printer.
    pub fn print(record: &log::Record) {
        #[cfg(feature="timestamp")]
        {
            std::println!("{:<5} [{}] {{{}:{}}} - {}",
                     record.level(),
                     get_date(),
                     record.file().unwrap_or("UNKNOWN"), record.line().unwrap_or(0),
                     record.args());

        }

        #[cfg(not(feature="timestamp"))]
        {
            std::println!("{:<5} {{{}:{}}} - {}",
                     record.level(),
                     record.file().unwrap_or("UNKNOWN"), record.line().unwrap_or(0),
                     record.args());
        }
    }
}
