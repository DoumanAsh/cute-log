#[test]
fn it_works() {
    const LOGGER: cute_log::Logger = cute_log::Logger::new();
    #[cfg(feature = "std")]
    LOGGER.set_log_env_or(cute_log::log::LevelFilter::Info);
    #[cfg(not(feature = "std"))]
    LOGGER.set_max_level(cute_log::log::LevelFilter::Info);
    let _ = LOGGER.set_logger();

    log::info!("it works!");

    log::debug!("it doesn't work!");
}
