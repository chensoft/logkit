//! Record represent a single log entry
use super::define::*;
use super::source::*;

/// Log Record
///
/// Each log is represented as a record, primarily utilized by the Logger. The Logger is responsible
/// for spawning the record, invoking plugin methods, and calling target methods on it. After use,
/// the record is recycled by the Logger to minimize allocation and enhance the speed of future
/// record creation.
#[derive(Debug, Clone)]
pub struct Record {
    level: Level,
    cache: Vec<u8>,
    source: Source,
}

impl Record {
    /// Create a new record
    ///
    /// The `capacity` argument specifies the initial capacity of the buffer.
    ///
    /// ```
    /// let mut record = logkit::Record::new(logkit::LEVEL_TRACE, 512, logkit::source!());
    /// assert_eq!(record.level(), logkit::LEVEL_TRACE);
    /// ```
    #[inline]
    pub fn new(level: Level, capacity: usize, source: Source) -> Self {
        let mut obj = Self {level, cache: Vec::with_capacity(capacity), source};
        obj.cache.push(b'{');
        obj
    }

    /// Reset record for reuse
    ///
    /// ```
    /// let mut record = logkit::Record::new(logkit::LEVEL_TRACE, 512, logkit::source!());
    /// assert_eq!(record.level(), logkit::LEVEL_TRACE);
    ///
    /// record = logkit::Record::set(record, logkit::LEVEL_ERROR, logkit::source!());
    /// assert_eq!(record.level(), logkit::LEVEL_ERROR);
    /// ```
    #[inline]
    pub fn set(mut record: Record, level: Level, source: Source) -> Self {
        record.level = level;
        record.cache.truncate(1); // preserve '{'
        record.source = source;
        record
    }

    /// Current record's log level
    #[inline]
    pub fn level(&self) -> Level {
        self.level
    }

    /// Current record's source info
    #[inline]
    pub fn source(&self) -> &Source {
        &self.source
    }

    /// Append field's key and value to record
    ///
    /// The order of fields is fixed, fields are stored in the order they are added.
    ///
    /// Note that duplicate fields are not filtered out.
    ///
    /// ```
    /// let mut record = logkit::Record::new(logkit::LEVEL_TRACE, 512, logkit::source!());
    /// record.append("pid", &12345);
    /// record.append("msg", &"think outside the box");
    /// record.finish();
    /// assert_eq!(String::from_utf8_lossy(record.buffer().as_slice()), "{\"pid\":12345,\"msg\":\"think outside the box\"}\n");
    /// ```
    #[inline]
    pub fn append(&mut self, key: &str, val: &impl Encode) -> &mut Self {
        key.encode(&mut self.cache);
        self.cache.push(b':');
        val.encode(&mut self.cache);
        self.cache.push(b',');
        self
    }

    /// Mark the end of the record
    ///
    /// ```
    /// let mut record = logkit::Record::new(logkit::LEVEL_TRACE, 512, logkit::source!());
    /// record.finish();
    /// assert_eq!(String::from_utf8_lossy(record.buffer().as_slice()), "{}\n");
    /// ```
    #[inline]
    pub fn finish(&mut self) {
        match self.cache.last_mut() {
            Some(val) if *val == b',' => *val = b'}',
            _ => self.cache.push(b'}'),
        }

        self.cache.push(b'\n');
    }

    /// Get the final buffer
    ///
    /// ```
    /// let mut record = logkit::Record::new(logkit::LEVEL_TRACE, 512, logkit::source!());
    /// record.append("msg", &"less is more");
    /// record.finish();
    /// assert_eq!(String::from_utf8_lossy(record.buffer().as_slice()), "{\"msg\":\"less is more\"}\n");
    /// ```
    #[inline]
    pub fn buffer(&self) -> &Vec<u8> {
        &self.cache
    }
}