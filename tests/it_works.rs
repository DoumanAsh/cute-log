extern crate log;
extern crate cute_log;

#[test]
fn it_works() {
    cute_log::Logger::print(&log::Record::builder().args(format_args!("it works!")).level(log::Level::Error).build());
    cute_log::Logger::print(&log::Record::builder().args(format_args!("it works!")).level(log::Level::Warn).build());
    cute_log::Logger::print(&log::Record::builder().args(format_args!("it works!")).build());
    cute_log::Logger::print(&log::Record::builder().args(format_args!("it works!")).level(log::Level::Debug).build());
    cute_log::Logger::print(&log::Record::builder().args(format_args!("it works!")).level(log::Level::Trace).build());
}
