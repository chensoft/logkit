//! mod types
// pub(crate) use std::collections::BTreeMap;

/// Level
pub type Level = i32;

pub const LEVEL_TRACE : Level = 0;
pub const LEVEL_DEBUG : Level = 1;
pub const LEVEL_INFO  : Level = 2;
pub const LEVEL_WARN  : Level = 3;
pub const LEVEL_ERROR : Level = 4;
pub const LEVEL_OFF   : Level = i32::MAX;

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