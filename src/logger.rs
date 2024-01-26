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
}

impl Logger {
    /// Create a new clean logger object
    ///
    /// ```
    /// let mut logger = logkit::Logger::new();
    /// logger.route(logkit::StderrTarget);
    /// logkit::set_default_logger(logger);
    /// ```
    pub const fn new() -> Self {
        Self {
            level: LEVEL_TRACE,
            alloc: 512,
            records: Mutex::new(RefCell::new(vec![])),
            plugins: vec![],
            targets: vec![],
        }
    }

    /// Create a logger object from env
    ///
    /// You can define the env value `RUST_LOG` to control the init log level
    ///
    /// ```shell,no_run
    /// # only allow logs equal to or higher than 'info'
    /// export RUST_LOG=info
    ///
    /// # only allow logs equal to or higher than 'warn'
    /// export RUST_LOG=warn
    ///
    /// # use numeric log level, **not recommended**
    /// export RUST_LOG=3
    /// ```
    ///
    /// ```
    /// std::env::set_var("RUST_LOG", "trace");
    /// assert_eq!(logkit::Logger::from_env().level(), logkit::LEVEL_TRACE);
    ///
    /// std::env::set_var("RUST_LOG", "debug");
    /// assert_eq!(logkit::Logger::from_env().level(), logkit::LEVEL_DEBUG);
    ///
    /// std::env::set_var("RUST_LOG", "info");
    /// assert_eq!(logkit::Logger::from_env().level(), logkit::LEVEL_INFO);
    ///
    /// std::env::set_var("RUST_LOG", "warn");
    /// assert_eq!(logkit::Logger::from_env().level(), logkit::LEVEL_WARN);
    ///
    /// std::env::set_var("RUST_LOG", "error");
    /// assert_eq!(logkit::Logger::from_env().level(), logkit::LEVEL_ERROR);
    ///
    /// std::env::set_var("RUST_LOG", logkit::LEVEL_INFO.to_string());
    /// assert_eq!(logkit::Logger::from_env().level(), logkit::LEVEL_INFO);
    ///
    /// let mut logger = logkit::Logger::from_env();
    /// logger.route(logkit::StderrTarget);
    /// logkit::set_default_logger(logger);
    /// ```
    pub fn from_env() -> Self {
        let mut obj = Logger::new();
        if let Ok(level) = std::env::var("RUST_LOG") {
            obj.level = match level.parse::<Level>() {
                Ok(val) => val,
                Err(_) => str_to_level(level.to_lowercase().as_str()),
            };
        }
        obj
    }

    /// Create a logger object with some predefined behaviours
    ///
    /// This object adds level and time fields to any records and includes a stack trace for
    /// records at the `ERROR` level. The output is directed to stderr by default.
    ///
    /// ```
    /// logkit::set_default_logger(logkit::Logger::from_def());
    /// ```
    pub fn from_def() -> Self {
        let mut obj = Logger::from_env();
        obj.mount(LevelPlugin);
        obj.mount(TimePlugin::from_millis());
        obj.mount(StackPlugin {level: LEVEL_ERROR});
        obj.route(StderrTarget);
        obj
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
    /// let mut logger = logkit::Logger::from_def();
    /// logger.limit(logkit::LEVEL_INFO);
    /// assert_eq!(logger.level(), logkit::LEVEL_INFO);
    /// ```
    pub fn limit(&mut self, level: Level) -> &mut Self {
        self.level = level;
        self
    }

    /// Check if the log level is equal to or higher than the limit
    ///
    /// ```
    /// let mut logger = logkit::Logger::from_def();
    /// logger.limit(logkit::LEVEL_INFO);
    ///
    /// assert_eq!(logger.allow(logkit::LEVEL_TRACE), false);
    /// assert_eq!(logger.allow(logkit::LEVEL_DEBUG), false);
    /// assert_eq!(logger.allow(logkit::LEVEL_INFO), true);
    /// assert_eq!(logger.allow(logkit::LEVEL_WARN), true);
    /// assert_eq!(logger.allow(logkit::LEVEL_ERROR), true);
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
    /// let mut logger = logkit::Logger::new();
    /// logger.mount(logkit::LevelPlugin);
    /// ```
    pub fn mount(&mut self, plugin: impl Plugin + 'static) -> &mut Self {
        self.plugins.push(Box::new(plugin));
        self
    }

    /// Uninstall a plugin
    ///
    /// ```
    /// let mut logger = logkit::Logger::from_def();
    /// logger.unmount(|t| t.as_any().downcast_ref::<logkit::LevelPlugin>().is_some());
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
    /// let mut logger = logkit::Logger::new();
    /// logger.route(logkit::StderrTarget);
    /// ```
    pub fn route(&mut self, target: impl Target + 'static) -> &mut Self {
        self.targets.push(Box::new(target));
        self
    }

    /// Remove a output target
    ///
    /// ```
    /// let mut logger = logkit::Logger::from_def();
    /// logger.unroute(|t| t.as_any().downcast_ref::<logkit::StderrTarget>().is_some());
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
    /// let mut logger = logkit::Logger::new();
    /// logger.route(logkit::StderrTarget);
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
    /// ```
    /// let mut logger = logkit::Logger::new();
    /// logger.route(logkit::StderrTarget);
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