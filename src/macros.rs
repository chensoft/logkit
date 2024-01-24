//! Built-in default logger and handy macros
use super::define::*;
use super::logger::*;

lazy_static! {
    static ref DEFAULT_LOGGER: RwLock<Logger> = RwLock::new(Logger::def());
}

/// The global default logger
///
/// This function provides a read-write lock. A read lock is necessary when spawning or flushing
/// a record. A write lock is required to change the log level, install a plugin, or add a target.
/// Be aware that the lock is non-reentrant. If you acquire the write lock, ensure it is released
/// before attempting to acquire the read lock.
///
/// ```
/// #[macro_use] extern crate logkit;
/// info!("current log level is {}", logkit::default_logger().read().level);
///
/// logkit::default_logger().write().level = logkit::LEVEL_INFO;
/// logkit::default_logger().write().route("stderr", Box::new(logkit::StderrTarget));
/// ```
#[inline]
pub fn default_logger() -> &'static RwLock<Logger> {
    &DEFAULT_LOGGER
}

/// Replace the default logger
pub fn set_default_logger(logger: Logger) {
    *(DEFAULT_LOGGER.write()) = logger;
}

/// Trace log
///
/// ```
/// #[macro_use] extern crate logkit;
///
/// trace!(); // outputs just a linebreak
/// trace!("plain message");
/// trace!("println-like message {} {}!", "Hello", "World");
/// trace!(name = "Alice", age = 20); // outputs only fields, no message
/// trace!(name = "Alice", age = 20; "separate fields and messages with semicolon");
/// trace!(name = "Alice", age = 20; "println-like message {} {}! with fields", "Hello", "World");
/// ```
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {{
        $crate::record!($crate::default_logger(), $crate::LEVEL_TRACE, $($arg)*)
    }};
}

/// Debug log
///
/// ```
/// #[macro_use] extern crate logkit;
///
/// debug!(); // outputs just a linebreak
/// debug!("plain message");
/// debug!("println-like message {} {}!", "Hello", "World");
/// debug!(name = "Alice", age = 20); // outputs only fields, no message
/// debug!(name = "Alice", age = 20; "separate fields and messages with semicolon");
/// debug!(name = "Alice", age = 20; "println-like message {} {}! with fields", "Hello", "World");
/// ```
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        $crate::record!($crate::default_logger(), $crate::LEVEL_DEBUG, $($arg)*)
    }};
}

/// Info log
///
/// ```
/// #[macro_use] extern crate logkit;
///
/// info!(); // outputs just a linebreak
/// info!("plain message");
/// info!("println-like message {} {}!", "Hello", "World");
/// info!(name = "Alice", age = 20); // outputs only fields, no message
/// info!(name = "Alice", age = 20; "separate fields and messages with semicolon");
/// info!(name = "Alice", age = 20; "println-like message {} {}! with fields", "Hello", "World");
/// ```
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        $crate::record!($crate::default_logger(), $crate::LEVEL_INFO, $($arg)*)
    }};
}

/// Warn log
///
/// ```
/// #[macro_use] extern crate logkit;
///
/// warn!(); // outputs just a linebreak
/// warn!("plain message");
/// warn!("println-like message {} {}!", "Hello", "World");
/// warn!(name = "Alice", age = 20); // outputs only fields, no message
/// warn!(name = "Alice", age = 20; "separate fields and messages with semicolon");
/// warn!(name = "Alice", age = 20; "println-like message {} {}! with fields", "Hello", "World");
/// ```
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        $crate::record!($crate::default_logger(), $crate::LEVEL_WARN, $($arg)*)
    }};
}

/// Error log
///
/// ```
/// #[macro_use] extern crate logkit;
///
/// error!(); // outputs just a linebreak
/// error!("plain message");
/// error!("println-like message {} {}!", "Hello", "World");
/// error!(name = "Alice", age = 20); // outputs only fields, no message
/// error!(name = "Alice", age = 20; "separate fields and messages with semicolon");
/// error!(name = "Alice", age = 20; "println-like message {} {}! with fields", "Hello", "World");
/// ```
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        $crate::record!($crate::default_logger(), $crate::LEVEL_ERROR, $($arg)*)
    }};
}

/// Default log
///
/// ```
/// #[macro_use] extern crate logkit;
///
/// record!(logkit::default_logger(), logkit::LEVEL_TRACE);
/// record!(logkit::default_logger(), logkit::LEVEL_TRACE, "I'm ready for adventure!");
/// record!(logkit::default_logger(), logkit::LEVEL_TRACE, "Hi {}! It's been {} years since our last trip together.", "Alice", 2);
/// record!(logkit::default_logger(), logkit::LEVEL_TRACE, name = "Alice", age = 20);
/// record!(logkit::default_logger(), logkit::LEVEL_TRACE, name = "Alice", age = 20; "I'm ready for adventure!");
/// record!(logkit::default_logger(), logkit::LEVEL_TRACE, name = "Alice", age = 20; "Hi {}! I know, time flies. I've visited {} countries since then.", "Bob", 3);
/// ```
#[macro_export]
macro_rules! record {
    // record!(logkit::LEVEL_TRACE);
    // {}
    ($log:expr, $lvl:expr $(,)?) => {{
        if let Some(record) = $log.read().spawn($lvl) {
            $log.read().flush(record);
        }
    }};

    // record!(logkit::LEVEL_TRACE, "I'm ready for adventure!");
    // {"msg":"I'm ready for adventure!"}
    ($log:expr, $lvl:expr, $fmt:literal) => {{
        $crate::record!($log, $lvl, $fmt, )
    }};

    // record!(logkit::LEVEL_TRACE, "Hi {}! It's been {} years since our last trip together.", "Alice", 2);
    // {"msg":"Hi Alice! It's been 2 years since our last trip together."}
    ($log:expr, $lvl:expr, $fmt:literal, $($arg:tt)*) => {{
        if let Some(mut record) = $log.read().spawn($lvl) {
            record.append("msg", format!($fmt, $($arg)*));
            $log.read().flush(record);
        }
    }};

    // record!(logkit::LEVEL_TRACE, name = "Alice", age = 20);
    // {"name":"Alice","age":20}
    ($log:expr, $lvl:expr, $($key:tt = $val:expr),*) => {{
        if let Some(mut record) = $log.read().spawn($lvl) {
            $(record.append(stringify!($key), $val);)*
            $log.read().flush(record);
        }
    }};

    // record!(logkit::LEVEL_TRACE, name = "Alice", age = 20; "I'm ready for adventure!");
    // {"msg":"I'm ready for adventure!","name":"Alice","age":20}
    ($log:expr, $lvl:expr, $($key:tt = $val:expr),+; $fmt:literal) => {{
        $crate::record!($log, $lvl, $($key = $val),+; $fmt, )
    }};

    // record!(logkit::LEVEL_TRACE, name = "Alice", age = 20; "Hi {}! I know, time flies. I've visited {} countries since then.", "Bob", 3);
    // {"msg":"Hi Bob! I know, time flies. I've visited 3 countries since then.","name":"Alice","age":20}
    ($log:expr, $lvl:expr, $($key:tt = $val:expr),+; $fmt:literal, $($arg:tt)*) => {{
        if let Some(mut record) = $log.read().spawn($lvl) {
            record.append("msg", format!($fmt, $($arg)*));
            $(record.append(stringify!($key), $val);)*

            $log.read().flush(record);
        }
    }};
}