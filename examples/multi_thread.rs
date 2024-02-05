#[macro_use] extern crate logkit;

#[tokio::main]
async fn main() {
    let mut logger = logkit::Logger::new(Some(&logkit::StdoutTarget));
    logger.mount(logkit::LevelPlugin);
    logger.mount(logkit::TimePlugin::from_millis());
    logkit::set_default_logger(logger);

    for i in 0..100 {
        tokio::task::spawn(async move {
            trace!("hello, this is a log with index {}", i);
        });
    }
}