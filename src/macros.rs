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
    ($lvl:expr) => {{
    }};

    // log!(logkit::LEVEL_TRACE, "I'm ready for adventure!");
    ($lvl:expr, $fmt:literal) => {{
    }};

    // log!(logkit::LEVEL_TRACE, "Hi {}! It's been {} years since our last trip together.", "Alice", 2);
    ($lvl:expr, $fmt:literal, $($arg:tt)*) => {{
    }};

    // log!(logkit::LEVEL_TRACE, name = "Alice", age = 20);
    ($lvl:expr, $($key:tt = $val:expr),*) => {{
    }};

    // log!(logkit::LEVEL_TRACE, name = "Alice", age = 20; "I'm ready for adventure!");
    ($lvl:expr, $($key:tt = $val:expr),+; $fmt:literal) => {{
    }};

    // log!(logkit::LEVEL_TRACE, name = "Alice", age = 20; "Hi {}! I know, time flies. I've visited {} countries since then.", "Bob", 3);
    ($lvl:expr, $($key:tt = $val:expr),+; $fmt:literal, $($arg:tt)*) => {{
    }};
}