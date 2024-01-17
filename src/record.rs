//! mod record
use super::define::*;

pub trait Format {} // todo useless?

pub struct Record {
    level: Level,
    capacity: usize,
    buffer: String,
}

impl Record {
    pub fn new(level: Level, capacity: usize) -> Self {
        let mut obj = Record { level, capacity, buffer: String::new() };
        obj.flush();
        obj
    }

    #[inline]
    pub fn level(&self) -> Level {
        self.level
    }

    #[inline]
    pub fn append(&mut self, key: &str, val: impl Appender) -> &mut Self {
        self.buffer.push('"'); // todo make a fn
        self.buffer.push_str(key);
        self.buffer.push_str("\":");
        val.append(&mut self.buffer);
        self
    }

    pub fn flush(&mut self) -> String {
        self.buffer.push('}');
        let ret = std::mem::replace(&mut self.buffer, String::with_capacity(self.capacity));
        self.buffer.push('{');
        ret
    }
}