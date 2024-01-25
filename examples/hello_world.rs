#[macro_use] extern crate logkit;

fn main() {
    // logging with the default logger
    trace!("hello, this is a trace log");
    debug!("hello, this is a debug log");
    info!(version = "0.1.0", commit = "3291cc60"; "this is a log with two string fields");
    warn!(address = "127.0.0.1", port = 3000; "this is a log with a string and a numeric field");
    error!("this is a log with a 'println' style string {}:{}", "127.0.0.1", 3000.0);

    // set default logger's log level
    logkit::default_logger().write().level = logkit::LEVEL_INFO;

    debug!("you can't see this log because the level is below 'info'");
    info!("only logs with a level equal to or higher than 'info' can be seen");
    error!("you can see this error log with stack trace");

    // remove logger's stack plugin
    logkit::default_logger().write().unmount(|t| t.as_any().downcast_ref::<logkit::StackPlugin>().is_some());

    error!("stack trace printing feature has been disabled");

    // create our own pid plugin
    pub struct PidPlugin { pub pid: u32 }

    impl logkit::Plugin for PidPlugin {
        fn post(&self, record: &mut logkit::Record) -> bool {
            record.append("pid", &self.pid);
            true
        }
    }

    logkit::default_logger().write().mount("pid", PidPlugin {pid: std::process::id()});

    info!("you will see this log with a process id");
}