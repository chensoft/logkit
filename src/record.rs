//! mod record
use super::define::*;

pub struct Record {
    level: Level,
    cache: Vec<u8>,
}

impl Record {
    pub fn new(level: Level, capacity: usize) -> Self {
        let mut obj = Self {level, cache: Vec::with_capacity(capacity)};
        obj.cache.push(b'{');
        obj
    }

    pub fn get(mut record: Record) -> Self {
        record.cache.truncate(1);
        record
    }

    pub fn level(&self) -> Level {
        self.level
    }

    pub fn append(&mut self, key: &str, val: impl Encode) -> &mut Self {
        key.encode(&mut self.cache);
        self.cache.push(b':');
        val.encode(&mut self.cache);
        self.cache.push(b',');
        self
    }

    pub fn finish(&mut self) {
        match self.cache.last_mut() {
            Some(val) if *val == b',' => *val = b'}',
            _ => self.cache.push(b'}'),
        }

        self.cache.push(b'\n');
    }

    pub fn buffer(&self) -> &Vec<u8> {
        &self.cache
    }
}