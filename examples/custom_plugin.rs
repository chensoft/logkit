#[macro_use] extern crate logkit;

fn main() {
    let mut logger = logkit::Logger::from_def();

    // remove the stack plugin
    logger.unmount(|t| t.as_any().downcast_ref::<logkit::StackPlugin>().is_some());

    // create a pid plugin
    pub struct PidPlugin { pub pid: u32 }

    impl logkit::Plugin for PidPlugin {
        fn post(&self, record: &mut logkit::Record) -> bool {
            record.append("pid", &self.pid);
            true
        }
    }

    logger.mount(PidPlugin {pid: std::process::id()});

    logkit::set_default_logger(logger);

    error!("stack trace printing feature has been disabled");
    info!("you will see this log with a process id");
}