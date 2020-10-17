#[test]
fn it_works() {
    const LOGGER: cute_log::Logger = cute_log::Logger::new();
    LOGGER.set_max_level(cute_log::log::LevelFilter::Info);
    let _ = LOGGER.set_logger();

    log::info!("it works!");

    log::debug!("it doesn't work!");
}
