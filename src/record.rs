//! mod record
use super::define::*;

pub struct Record {
    level: Level,
    cache: Vec<u8>,
}

impl Record {
    #[inline]
    pub fn new(level: Level, capacity: usize) -> Self {
        let mut obj = Self {level, cache: Vec::with_capacity(capacity)};
        obj.cache.push(b'{');
        obj
    }

    #[inline]
    pub fn get(mut record: Record) -> Self {
        record.cache.truncate(1);
        record
    }

    #[inline]
    pub fn level(&self) -> Level {
        self.level
    }

    #[inline]
    pub fn append(&mut self, key: &str, val: impl Encode) -> &mut Self {
        key.encode(&mut self.cache);
        self.cache.push(b':');
        val.encode(&mut self.cache);
        self.cache.push(b',');
        self
    }

    #[inline]
    pub fn finish(&mut self) {
        match self.cache.last_mut() {
            Some(val) if *val == b',' => *val = b'}',
            _ => self.cache.push(b'}'),
        }

        self.cache.push(b'\n');
    }

    #[inline]
    pub fn buffer(&self) -> &Vec<u8> {
        &self.cache
    }
}