#[macro_use] extern crate logkit;

fn main() {
    let mut logger = logkit::Logger::new(Some(&logkit::StdoutTarget));
    logger.limit(logkit::LEVEL_INFO);
    logger.mount(logkit::LevelPlugin);
    logkit::set_default_logger(logger);

    debug!("you can't see this log because the level is below 'info'");
    info!("only logs with a level equal to or higher than 'info' can be seen");
    error!("you can see this error log with stack trace");
}