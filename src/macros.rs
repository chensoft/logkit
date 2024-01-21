//! mod macros
use super::define::*;
use super::logger::*;
use super::plugin::*;
use super::target::*;

lazy_static! {
    static ref DEFAULT_LOGGER: RwLock<Logger> = RwLock::new({
        let mut obj = Logger::new();
        obj.mount(Box::new(LevelPlugin));
        obj.mount(Box::new(TimePlugin::from_millis()));
        obj.mount(Box::new(StackPlugin {level: LEVEL_ERROR}));
        obj.route(Box::new(StdoutTarget));
        obj
    });
}

pub fn default_logger() -> &'static RwLock<Logger> {
    &DEFAULT_LOGGER
}

pub fn set_default_logger(logger: Logger) {
    *(DEFAULT_LOGGER.write()) = logger;
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {{
        $crate::record!($crate::LEVEL_TRACE, $($arg)*)
    }};
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        $crate::record!($crate::LEVEL_DEBUG, $($arg)*)
    }};
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        $crate::record!($crate::LEVEL_INFO, $($arg)*)
    }};
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        $crate::record!($crate::LEVEL_WARN, $($arg)*)
    }};
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        $crate::record!($crate::LEVEL_ERROR, $($arg)*)
    }};
}

#[macro_export]
macro_rules! record {
    // record!(logkit::LEVEL_TRACE);
    // {}
    ($lvl:expr) => {{
        if let Some(record) = $crate::default_logger().read().spawn($lvl) {
            $crate::default_logger().read().write(record);
        }
    }};

    // record!(logkit::LEVEL_TRACE, "I'm ready for adventure!");
    // {"msg":"I'm ready for adventure!"}
    ($lvl:expr, $fmt:literal) => {{
        $crate::record!($lvl, $fmt, )
    }};

    // record!(logkit::LEVEL_TRACE, "Hi {}! It's been {} years since our last trip together.", "Alice", 2);
    // {"msg":"Hi Alice! It's been 2 years since our last trip together."}
    ($lvl:expr, $fmt:literal, $($arg:tt)*) => {{
        if let Some(mut record) = $crate::default_logger().read().spawn($lvl) {
            record.append("msg", format!($fmt, $($arg)*));
            $crate::default_logger().read().write(record);
        }
    }};

    // record!(logkit::LEVEL_TRACE, name = "Alice", age = 20);
    // {"name":"Alice","age":20}
    ($lvl:expr, $($key:tt = $val:expr),*) => {{
        if let Some(mut record) = $crate::default_logger().read().spawn($lvl) {
            $(record.append(stringify!($key), $val);)*
            $crate::default_logger().read().write(record);
        }
    }};

    // record!(logkit::LEVEL_TRACE, name = "Alice", age = 20; "I'm ready for adventure!");
    // {"msg":"I'm ready for adventure!","name":"Alice","age":20}
    ($lvl:expr, $($key:tt = $val:expr),+; $fmt:literal) => {{
        $crate::record!($lvl, $($key = $val),+; $fmt, )
    }};

    // record!(logkit::LEVEL_TRACE, name = "Alice", age = 20; "Hi {}! I know, time flies. I've visited {} countries since then.", "Bob", 3);
    // {"msg":"Hi Bob! I know, time flies. I've visited 3 countries since then.","name":"Alice","age":20}
    ($lvl:expr, $($key:tt = $val:expr),+; $fmt:literal, $($arg:tt)*) => {{
        if let Some(mut record) = $crate::default_logger().read().spawn($lvl) {
            record.append("msg", format!($fmt, $($arg)*));
            $(record.append(stringify!($key), $val);)*

            $crate::default_logger().read().write(record);
        }
    }};
}

#[test]
fn test() {
    use crate as logkit;

    record!(logkit::LEVEL_TRACE);
    record!(logkit::LEVEL_TRACE, "I'm ready for adventure!");
    record!(logkit::LEVEL_TRACE, "Hi {}! It's been {} years since our last trip together.", "Alice", 2);
    record!(logkit::LEVEL_TRACE, name = "Alice", age = 20);
    record!(logkit::LEVEL_TRACE, name = "Alice", age = 20; "I'm ready for adventure!");
    record!(logkit::LEVEL_TRACE, name = "Alice", age = 20; "Hi {}! I know, time flies. I've visited {} countries since then.", "Bob", 3);
}