extern crate log;
extern crate cute_log;

#[test]
fn it_works() {
    let record = log::Record::builder().args(format_args!("it works!")).build();

    cute_log::Logger::print(&record);
}
