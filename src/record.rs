//! mod record
use super::define::*;
use super::encode::*;
use super::target::*;

pub struct Record {
    level: Level,
    cache: Vec<u8>,
}

impl Record {
    pub fn new(level: Level, capacity: usize) -> Self {
        let mut obj = Record {level, cache: Vec::with_capacity(capacity)};
        obj.cache.push(b'{');
        obj
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
    pub fn flush(&mut self, dest: &(impl Target + ?Sized)) {
        match self.cache.last_mut() {
            Some(val) if *val == b',' => *val = b'}',
            _ => self.cache.push(b'}'),
        }

        self.cache.push(b'\n');
        dest.write(&self.cache);
        self.cache.truncate(1);
    }
}