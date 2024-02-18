#[macro_use] extern crate logkit;

fn main() {
    // show the stack trace
    let mut logger = logkit::Logger::new(Some(&logkit::StdoutTarget));
    logger.mount(logkit::StackPlugin::from_level(logkit::LEVEL_ERROR));
    logkit::set_default_logger(logger);

    info!("set the env `RUST_BACKTRACE=1` before running this example");
    error!("you will see a stack trace if you enable the RUST_BACKTRACE");

    // remove the stack plugin
    let mut logger = logkit::Logger::new(Some(&logkit::StdoutTarget));
    logger.mount(logkit::StackPlugin::from_level(logkit::LEVEL_ERROR));
    logger.unmount(|t| t.as_any().downcast_ref::<logkit::StackPlugin>().is_some());
    logkit::set_default_logger(logger);

    error!("stack trace printing feature has been disabled");

    // create a pid plugin
    pub struct PidPlugin { pub pid: u32 }

    impl logkit::Plugin for PidPlugin {
        fn post(&self, record: &mut logkit::Record) -> bool {
            record.append("pid", &self.pid);
            true
        }
    }

    let mut logger = logkit::Logger::new(Some(&logkit::StdoutTarget));
    logger.mount(PidPlugin {pid: std::process::id()});
    logkit::set_default_logger(logger);

    info!("you will see this log with a process id");
}