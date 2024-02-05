#[macro_use] extern crate logkit;

#[tokio::main]
async fn main() {
    let mut logger = logkit::Logger::new(Some(&logkit::StdoutTarget));
    logger.mount(logkit::LevelPlugin);
    logger.mount(logkit::TimePlugin::from_millis());
    logkit::set_default_logger(logger);

    let mut handles = vec![];

    for i in 0..100 {
        handles.push(tokio::task::spawn(async move {
            trace!("hello, this is a log with index {}", i);
        }));
    }

    futures::future::join_all(handles).await;
}