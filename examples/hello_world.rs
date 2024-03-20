#[macro_use] extern crate logkit;

fn main() {
    let mut logger = logkit::Logger::new(Some(&logkit::StdoutTarget));
    logger.mount(logkit::LevelPlugin);
    logger.mount(logkit::TimePlugin::from_millis());
    logger.mount(logkit::SourcePlugin);
    logkit::set_default_logger(logger);

    trace!("hello, this is a trace log");
    debug!("hello, this is a debug log");
    info!(version = "0.1.0", commit = "3291cc60"; "this is a log with two string fields");
    warn!(address = "127.0.0.1", port = 3000; "this is a log with a string and a numeric field");
    error!("this is a log with a 'println' style string {}:{}", "127.0.0.1", 3000.0);
}