//! The central struct designed for managing logging tasks
use super::define::*;
use super::record::*;
use super::plugin::*;
use super::target::*;

/// The Logger
///
/// Responsible for setting the log level, spawning log records, and managing plugins, targets,
/// and all other logging functionalities.
pub struct Logger {
    level: Level, // log level limit
    alloc: usize, // record init capacity

    records: Mutex<RefCell<Vec<Record>>>, // records pool
    plugins: Vec<Box<dyn AnyPlugin>>,     // middlewares
    targets: Vec<Box<dyn AnyTarget>>,     // output targets
    default: Option<&'static dyn Target>, // default output
}

impl Logger {
    /// Create a logger with a default static target
    ///
    /// Note that the default target can't be deleted.
    ///
    /// ```
    /// let logger = logkit::Logger::new(Some(&logkit::StderrTarget));
    /// logkit::set_default_logger(logger);
    /// ```
    pub const fn new(default: Option<&'static dyn Target>) -> Self {
        Self {
            level: LEVEL_TRACE,
            alloc: 512,
            records: Mutex::new(RefCell::new(vec![])),
            plugins: vec![],
            targets: vec![],
            default,
        }
    }

    /// Create a logger without output target
    ///
    /// ```
    /// logkit::set_default_logger(logkit::Logger::nop());
    /// ```
    pub const fn nop() -> Self {
        Self::new(None)
    }

    /// Get current log level
    ///
    /// ```
    /// assert_eq!(logkit::default_logger().level(), logkit::LEVEL_TRACE);
    /// ```
    pub fn level(&self) -> Level {
        self.level
    }

    /// Set current log level
    ///
    /// ```
    /// let mut logger = logkit::Logger::new(Some(&logkit::StderrTarget));
    /// logger.limit(logkit::LEVEL_INFO);
    /// logkit::set_default_logger(logger);
    /// assert_eq!(logkit::default_logger().level(), logkit::LEVEL_INFO);
    /// ```
    pub fn limit(&mut self, level: Level) -> &mut Self {
        self.level = level;
        self
    }

    /// Check if the log level is equal to or higher than the limit
    ///
    /// ```
    /// let mut logger = logkit::Logger::new(Some(&logkit::StderrTarget));
    /// logger.limit(logkit::LEVEL_INFO);
    /// logkit::set_default_logger(logger);
    ///
    /// assert_eq!(logkit::default_logger().allow(logkit::LEVEL_TRACE), false);
    /// assert_eq!(logkit::default_logger().allow(logkit::LEVEL_DEBUG), false);
    /// assert_eq!(logkit::default_logger().allow(logkit::LEVEL_INFO), true);
    /// assert_eq!(logkit::default_logger().allow(logkit::LEVEL_WARN), true);
    /// assert_eq!(logkit::default_logger().allow(logkit::LEVEL_ERROR), true);
    /// ```
    #[inline]
    pub fn allow(&self, level: Level) -> bool {
        level >= self.level
    }

    /// Install a plugin for records
    ///
    /// A plugin acts as middleware for logs. For more details, refer to `plugin.rs`.
    ///
    /// ```
    /// let mut logger = logkit::Logger::new(Some(&logkit::StderrTarget));
    /// logger.mount(logkit::LevelPlugin);
    /// logger.mount(logkit::TimePlugin::from_millis());
    /// logkit::set_default_logger(logger);
    /// ```
    pub fn mount(&mut self, plugin: impl Plugin) -> &mut Self {
        self.plugins.push(Box::new(plugin));
        self
    }

    /// Uninstall a plugin
    ///
    /// ```
    /// let mut logger = logkit::Logger::new(Some(&logkit::StderrTarget));
    /// logger.mount(logkit::LevelPlugin);
    /// logger.unmount(|t| t.as_any().downcast_ref::<logkit::LevelPlugin>().is_some());
    /// logkit::set_default_logger(logger);
    /// ```
    pub fn unmount(&mut self, del: impl Fn(&Box<dyn AnyPlugin>) -> bool) -> &mut Self {
        self.plugins.retain(|plugin| !del(plugin));
        self
    }

    /// Add a output target for records
    ///
    /// Multiple targets are supported, allowing you to output a single record to various places.
    ///
    /// ```
    /// let mut logger = logkit::Logger::new(None);
    /// logger.route(logkit::StderrTarget);
    /// logkit::set_default_logger(logger);
    /// ```
    pub fn route(&mut self, target: impl Target) -> &mut Self {
        self.targets.push(Box::new(target));
        self
    }

    /// Remove a output target
    ///
    /// ```
    /// let mut logger = logkit::Logger::new(None);
    /// logger.route(logkit::StderrTarget);
    /// logger.unroute(|t| t.as_any().downcast_ref::<logkit::StderrTarget>().is_some());
    /// logkit::set_default_logger(logger);
    /// ```
    pub fn unroute(&mut self, del: impl Fn(&Box<dyn AnyTarget>) -> bool) -> &mut Self {
        self.targets.retain(|target| !del(target));
        self
    }

    /// Create a new log record
    ///
    /// Internally, each log is represented by a record, which contains level information and
    /// a cached buffer. You can append numerous fields to a record. The println-like message is
    /// also treated as a normal field with the key named `msg`.
    ///
    /// ```
    /// let logger = logkit::Logger::new(Some(&logkit::StderrTarget));
    ///
    /// if let Some(mut record) = logger.spawn(logkit::LEVEL_TRACE) {
    ///     record.append("hello", &"world");
    ///     record.finish();
    ///     assert_eq!(String::from_utf8_lossy(record.buffer().as_slice()), "{\"hello\":\"world\"}\n")
    /// }
    /// ```
    #[inline]
    pub fn spawn(&self, level: Level) -> Option<Record> {
        if !self.allow(level) {
            return None;
        }

        let record = {
            let guard = self.records.lock();
            let mut array = guard.borrow_mut();
            array.pop()
        };

        let mut record = match record {
            None => Record::new(level, self.alloc),
            Some(val) => Record::set(val, level),
        };

        for plugin in &self.plugins {
            if !plugin.pre(&mut record) {
                self.reuse(record);
                return None;
            }
        }

        Some(record)
    }

    /// Finish and output a record
    ///
    /// The `post` method of plugins will be called. If you wish to prevent output to targets,
    /// simply return `false`. Once the `finish` method is invoked, the record will be directed
    /// to all installed targets for output.
    ///
    /// Note that the default target is always invoked first.
    ///
    /// ```
    /// let logger = logkit::Logger::new(Some(&logkit::StderrTarget));
    ///
    /// if let Some(mut record) = logger.spawn(logkit::LEVEL_TRACE) {
    ///     record.append("msg", &"this log will be directed to stderr");
    ///     logger.flush(record);
    /// }
    /// ```
    #[inline]
    pub fn flush(&self, mut record: Record) {
        for plugin in &self.plugins {
            if !plugin.post(&mut record) {
                self.reuse(record);
                return;
            }
        }

        record.finish();

        if let Some(target) = self.default {
            target.write(record.buffer());
        }

        for target in &self.targets {
            target.write(record.buffer());
        }

        self.reuse(record);
    }

    /// Places the record back into the object pool for reuse
    ///
    /// The `flush` method calls this function automatically, so typically you don't need to
    /// invoke it manually.
    #[inline]
    pub fn reuse(&self, record: Record) {
        let guard = self.records.lock();
        guard.borrow_mut().push(record);
    }
}