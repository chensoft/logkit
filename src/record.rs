//! mod record
use super::define::*;

pub struct Record {
    level: Level,
    cache: Vec<u8>,
}

impl Record {
    pub fn level(&self) -> Level {
        self.level
    }

    pub fn reset(&mut self, level: Level, capacity: usize) {
        self.level = level;
        self.cache.reserve(std::cmp::max(0, capacity - self.cache.capacity()));
        self.cache.truncate(1);
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

pub type RecordWrapper<'a> = RefGuard<'a, RecordAllocator, Record>;

pub struct RecordAllocator;

impl PoolAllocator<Record> for RecordAllocator {
    #[inline]
    fn reset(&self, _obj: &mut Record) {}

    #[inline]
    fn allocate(&self) -> Record {
        let mut obj = Record {level: LEVEL_OFF, cache: vec![]};
        obj.cache.push(b'{');
        obj
    }

    #[inline]
    fn is_valid(&self, _obj: &Record) -> bool {
        true
    }
}

lazy_static! {
    pub static ref RECORD_POOL: Pool<RecordAllocator, Record> = Pool::new(128, RecordAllocator);
}