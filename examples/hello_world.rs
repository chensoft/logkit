#[macro_use] extern crate logkit;

fn main() {
    // logging with default logger
    trace!("hello, this is a trace log");
    debug!("hello, this is a debug log");
    info!(version = "0.1.0", commit = "3291cc60"; "server is started");
    warn!(address = "127.0.0.1", port = 3000; "listen and serve");
    error!("address already in use {}:{}", "127.0.0.1", 3000.0);

    // set default logger's log level
    logkit::default_logger_mut().level = logkit::LEVEL_INFO;

    debug!("you can't see this log because the level is below 'info'");
    info!("only logs with a level equal to or higher than 'info' will be printed");
    error!("you can see this error log with stack trace");

    // remove logger's plugin
    logkit::default_logger_mut().unmount("stack");

    error!("stack trace printing feature has been disabled");

    // create our own plugin
    struct PidPlugin;

    impl logkit::Plugin for PidPlugin {
        fn post(&self, key: &str, record: &mut logkit::Record) -> bool {
            record.append(key, std::process::id());
            true
        }
    }

    logkit::default_logger_mut().mount("pid", Box::new(PidPlugin));

    info!("you will see logs with process id");
}