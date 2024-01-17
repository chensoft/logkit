//! mod consts
pub(crate) use parking_lot::RwLock;

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

/// todo rename Appender
pub trait Appender {
    fn append(&self, buffer: &mut String);
}

impl Appender for &str {
    fn append(&self, buffer: &mut String) {
        buffer.push_str(self);
    }
}

impl Appender for String {
    fn append(&self, buffer: &mut String) {
        buffer.push_str(&self);
    }
}

impl Appender for i32 {
    fn append(&self, buffer: &mut String) {
        buffer.push_str(&self.to_string());
    }
}