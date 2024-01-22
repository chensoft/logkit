//! mod record
use super::define::*;

#[derive(Default, Debug, Clone)]
pub struct Record {
    level: Level,
    cache: Vec<u8>,
}

impl Record {
    #[inline]
    pub fn reset(&mut self, level: Level, capacity: usize) {
        self.level = level;
        self.cache.reserve(std::cmp::max(0, capacity - self.cache.len()));
        self.cache.clear();
        self.cache.push(b'{');
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