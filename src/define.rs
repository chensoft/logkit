//! Logging levels and Encode trait
pub(crate) use std::io::Write;
pub(crate) use std::borrow::Cow;
pub(crate) use std::cell::RefCell;
pub(crate) use indexmap::IndexMap;
pub(crate) use parking_lot::RwLock;
pub(crate) use parking_lot::ReentrantMutex;
pub(crate) use parking_lot::RwLockReadGuard;
pub(crate) use parking_lot::RwLockWriteGuard;

/// Trait
pub use encoder::json::Encode;

/// Level
pub type Level = i32;

pub const LEVEL_TRACE : Level = 0;
pub const LEVEL_DEBUG : Level = 1;
pub const LEVEL_INFO  : Level = 2;
pub const LEVEL_WARN  : Level = 3;
pub const LEVEL_ERROR : Level = 4;
pub const LEVEL_OFF   : Level = i32::MAX;

pub fn level_to_str(level: Level) -> Option<&'static str> {
    match level {
        LEVEL_TRACE => Some("trace"),
        LEVEL_DEBUG => Some("debug"),
        LEVEL_INFO => Some("info"),
        LEVEL_WARN => Some("warn"),
        LEVEL_ERROR => Some("error"),
        _ => None,
    }
}

pub fn str_to_level(level: &str) -> Level {
    match level {
        "trace" => LEVEL_TRACE,
        "debug" => LEVEL_DEBUG,
        "info" => LEVEL_INFO,
        "warn" => LEVEL_WARN,
        "error" => LEVEL_ERROR,
        _ => LEVEL_OFF,
    }
}