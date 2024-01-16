//! mod record
use super::consts::*;

pub trait Format {} // todo useless?

pub struct Record {
    pub capacity: usize,
    pub buffer: String,
}

impl Record {
    pub fn new(capacity: usize) -> Self {
        let mut obj = Record { capacity, buffer: String::new() };
        obj.flush();
        obj
    }

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