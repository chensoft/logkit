#[macro_use] extern crate logkit;

fn main() -> std::io::Result<()> {
    let mut logger = logkit::Logger::new(Some(&logkit::StdoutTarget));
    logger.mount(logkit::TimePlugin::from_millis());
    logger.mount(logkit::LevelPlugin);

    let mut sample = std::env::temp_dir();
    sample.push("sample.log");
    logger.route(logkit::FileTarget::new(sample)?);

    logkit::set_default_logger(logger);

    let mut handles = vec![];

    for i in 0..100 {
        handles.push(std::thread::spawn(move || {
            trace!("hello, this is a log with index {}", i);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}