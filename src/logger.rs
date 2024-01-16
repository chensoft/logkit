//! mod logger
use super::consts::*;
use super::record::*;

pub struct Logger {
    pub level: Level,
    // todo targets
}

impl Logger {
    pub fn new() -> Self {
        Logger {level: LEVEL_TRACE} // todo read env
    }

    #[inline]
    pub fn record(&self, level: Level) -> Option<Record> {
        match level >= self.level {
            true => Some(Record::new(1024)), // todo config
            false => None,
        }
    }
}