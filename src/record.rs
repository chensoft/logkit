//! mod record
use super::define::*;
use super::encode::*;

pub struct Record {
    level: Level,
    cache: Vec<u8>,
}

impl Record {
    pub fn get(level: Level, capacity: usize) -> Self {
        let mut obj = Record {level, cache: Vec::with_capacity(capacity)};
        obj.cache.push(b'{');
        obj
    }

    pub fn put(mut record: Record) {
        // todo reuse record
        record.cache.truncate(1);
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