//! Built-in default logger and handy macros
use super::logger::*;
use super::target::*;

static mut DEFAULT_LOGGER: Logger = Logger::new(Some(&StderrTarget));

/// The global default logger
///
/// This logger uses stderr as its default output target.
#[inline]
#[allow(unknown_lints)]
#[allow(static_mut_ref)]
pub fn default_logger() -> &'static Logger {
    unsafe { &DEFAULT_LOGGER }
}

/// Replace the default logger
///
/// # Safety
///
/// This function is not thread-safe and is typically called at a very early stage of a program,
/// such as at the beginning of the `main` function.
///
/// **MAKE SURE NO OTHER THREADS ARE ACCESSING IT.**
///
/// ```
/// #[macro_use] extern crate logkit;
///
/// fn main() {
///     // use stdout as the default target
///     let logger = logkit::Logger::new(Some(&logkit::StdoutTarget));
///     logkit::set_default_logger(logger);
///
///     info!("Hello World!");
/// }
/// ```
pub fn set_default_logger(logger: Logger) {
    unsafe { DEFAULT_LOGGER = logger; }
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
#[cfg(feature = "trace")]
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {{
        $crate::record!($crate::default_logger(), $crate::LEVEL_TRACE, $($arg)*)
    }};
}

#[cfg(not(feature = "trace"))]
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {{
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
#[cfg(feature = "debug")]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        $crate::record!($crate::default_logger(), $crate::LEVEL_DEBUG, $($arg)*)
    }};
}

#[cfg(not(feature = "debug"))]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
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
#[cfg(feature = "info")]
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        $crate::record!($crate::default_logger(), $crate::LEVEL_INFO, $($arg)*)
    }};
}

#[cfg(not(feature = "info"))]
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
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
#[cfg(feature = "warn")]
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        $crate::record!($crate::default_logger(), $crate::LEVEL_WARN, $($arg)*)
    }};
}

#[cfg(not(feature = "warn"))]
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
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
#[cfg(feature = "error")]
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        $crate::record!($crate::default_logger(), $crate::LEVEL_ERROR, $($arg)*)
    }};
}

#[cfg(not(feature = "error"))]
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
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
///
/// record!(logkit::default_logger(), logkit::LEVEL_TRACE, "trailing comma", );
/// record!(logkit::default_logger(), logkit::LEVEL_TRACE, "println-like with trailing comma {} {}!", "Hello", "World", );
/// record!(logkit::default_logger(), logkit::LEVEL_TRACE, name = "Alice", age = 20, ); // fields with trailing comma
/// record!(logkit::default_logger(), logkit::LEVEL_TRACE, name = "Alice", age = 20; "fields and message with trailing comma", );
/// record!(logkit::default_logger(), logkit::LEVEL_TRACE, name = "Alice", age = 20; "println-like with fields and trailing comma {}", "Hello", );
///
/// let mut name = "Alice";
/// record!(logkit::default_logger(), logkit::LEVEL_TRACE, name = name, age = 10);
/// record!(logkit::default_logger(), logkit::LEVEL_TRACE, name = name, age = 20); // field formatted twice
/// ```
#[macro_export]
macro_rules! record {
    // record!(logkit::LEVEL_TRACE);
    // {}
    ($log:expr, $lvl:expr $(,)?) => {{
        if let Some(record) = $log.spawn($lvl) {
            $log.flush(record);
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
        if let Some(mut record) = $log.spawn($lvl) {
            record.append("msg", &format!($fmt, $($arg)*));
            $log.flush(record);
        }
    }};

    // record!(logkit::LEVEL_TRACE, name = "Alice", age = 20);
    // {"name":"Alice","age":20}
    ($log:expr, $lvl:expr, $($key:tt = $val:expr),+ $(,)?) => {{
        if let Some(mut record) = $log.spawn($lvl) {
            $(record.append(stringify!($key), &$val);)+
            $log.flush(record);
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
        if let Some(mut record) = $log.spawn($lvl) {
            record.append("msg", &format!($fmt, $($arg)*));
            $(record.append(stringify!($key), &$val);)*

            $log.flush(record);
        }
    }};
}