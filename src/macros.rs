//! mod macros
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {{
        $crate::log!($crate::LEVEL_TRACE, $($arg)*)
    }};
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        $crate::log!($crate::LEVEL_DEBUG, $($arg)*)
    }};
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        $crate::log!($crate::LEVEL_INFO, $($arg)*)
    }};
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        $crate::log!($crate::LEVEL_WARN, $($arg)*)
    }};
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        $crate::log!($crate::LEVEL_ERROR, $($arg)*)
    }};
}

#[macro_export]
macro_rules! log {
    // log!(logkit::LEVEL_TRACE);
    // {}
    ($lvl:expr) => {{
        $crate::Logger::new().record($lvl);
    }};

    // log!(logkit::LEVEL_TRACE, "I'm ready for adventure!");
    // {"msg":"I'm ready for adventure!"}
    ($lvl:expr, $fmt:literal) => {{
        $crate::log!($lvl, $fmt, );
    }};

    // log!(logkit::LEVEL_TRACE, "Hi {}! It's been {} years since our last trip together.", "Alice", 2);
    // {"msg":"Hi Alice! It's been 2 years since our last trip together."}
    ($lvl:expr, $fmt:literal, $($arg:tt)*) => {{
        if let Some(mut record) = $crate::Logger::new().record($lvl) {
            record.append("msg", format!($fmt, $($arg)*));
        }
    }};

    // log!(logkit::LEVEL_TRACE, name = "Alice", age = 20);
    // {"name":"Alice","age":20}
    ($lvl:expr, $($key:tt = $val:expr),*) => {{
        if let Some(mut record) = $crate::Logger::new().record($lvl) {
            $(record.append(stringify!($key), $val);)*
        }
    }};

    // log!(logkit::LEVEL_TRACE, name = "Alice", age = 20; "I'm ready for adventure!");
    // {"name":"Alice","age":20,"msg":"I'm ready for adventure!"}
    ($lvl:expr, $($key:tt = $val:expr),+; $fmt:literal) => {{
        $crate::log!($lvl, $($key = $val),+; $fmt, )
    }};

    // log!(logkit::LEVEL_TRACE, name = "Alice", age = 20; "Hi {}! I know, time flies. I've visited {} countries since then.", "Bob", 3);
    // {"name":"Alice","age":20,"msg":"Hi Bob! I know, time flies. I've visited 3 countries since then."}
    ($lvl:expr, $($key:tt = $val:expr),+; $fmt:literal, $($arg:tt)*) => {{
        if let Some(mut record) = $crate::Logger::new().record($lvl) {
            $(record.append(stringify!($key), $val);)*
            record.append("msg", format!($fmt, $($arg)*));
        }
    }};
}

#[test]
fn test() {
    use super::types::*;

    log!(LEVEL_TRACE);
    log!(LEVEL_TRACE, "I'm ready for adventure!");
    log!(LEVEL_TRACE, "Hi {}! It's been {} years since our last trip together.", "Alice", 2);
    log!(LEVEL_TRACE, name = "Alice", age = 20);
    log!(LEVEL_TRACE, name = "Alice", age = 20; "I'm ready for adventure!");
    log!(LEVEL_TRACE, name = "Alice", age = 20; "Hi {}! I know, time flies. I've visited {} countries since then.", "Bob", 3);
}