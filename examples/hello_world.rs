#[macro_use] extern crate logkit;

fn main() {
    // logging with default logger
    trace!("main()");
    debug!("main function was called");
    info!(version = "0.1.0", commit = "3291cc60"; "server is started");
    warn!(address = "127.0.0.1", port = 3000; "listen and serve");
    error!("address already in use {}:{}", "127.0.0.1", 3000.0);

    // set default logger's log level
    logkit::default_logger_mut().level = logkit::LEVEL_WARN;

    info!("you can't see this log record");
    warn!("only logs with a level of 'warn' or higher are visible");
    error!("you can see this error log with stack trace");
}