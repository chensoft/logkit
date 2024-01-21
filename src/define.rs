//! mod consts
pub(crate) use std::io::Write;
pub(crate) use std::cell::RefCell;
pub(crate) use opool::Pool;
pub(crate) use opool::RefGuard;
pub(crate) use opool::PoolAllocator;
pub(crate) use parking_lot::RwLock;
pub(crate) use parking_lot::ReentrantMutex;

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

pub fn level_to_str(level: Level) -> &'static str {
    match level {
        LEVEL_TRACE => "trace",
        LEVEL_DEBUG => "debug",
        LEVEL_INFO => "info",
        LEVEL_WARN => "warn",
        LEVEL_ERROR => "error",
        _ => "",
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