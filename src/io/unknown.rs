impl crate::Logger {
    #[inline(always)]
    ///Dummy printer
    pub fn print_fmt(_: core::fmt::Arguments<'_>) {
    }

    #[inline(always)]
    ///Dummy printer
    pub fn print(_record: &log::Record) {
    }
}
