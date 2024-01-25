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
    pub level: Level, // log level limit
    pub alloc: usize, // record init capacity

    records: Mutex<RefCell<Vec<Record>>>, // records pool
    plugins: IndexMap<Cow<'static, str>, Box<dyn Plugin>>, // middlewares
    targets: IndexMap<Cow<'static, str>, Box<dyn Target>>, // output targets
}

impl Logger {
    /// Create a new clean logger object
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
    pub fn new() -> Self {
        let mut obj = Self {
            level: LEVEL_TRACE,
            alloc: 512,
            records: Mutex::new(RefCell::new(vec![])),
            plugins: IndexMap::new(),
            targets: IndexMap::new()
        };

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
    /// records at the `ERROR` level. The output is directed to stdout by default.
    ///
    /// Note that this configuration is used by the global default logger. If you want to modify
    /// the global default logger, you can use the following example code:
    ///
    /// ```
    /// // unmount the stack plugin
    /// logkit::default_logger().write().unmount("stack");
    ///
    /// // use nanoseconds time plugin
    /// logkit::default_logger().write().mount("time", logkit::TimePlugin::from_nanos());
    ///
    /// // change default target to stderr
    /// logkit::default_logger().write().route("default", logkit::StderrTarget);
    /// ```
    pub fn def() -> Self {
        let mut obj = Logger::new();
        obj.mount("level", LevelPlugin);
        obj.mount("time", TimePlugin::from_millis());
        obj.mount("stack", StackPlugin {level: LEVEL_ERROR});
        obj.route("default", StdoutTarget);
        obj
    }

    /// Install a plugin for records
    ///
    /// A plugin acts as middleware for logs. For more details, refer to `plugin.rs`.
    ///
    /// ```
    /// let mut logger = logkit::Logger::new();
    /// logger.mount("level", logkit::LevelPlugin);
    /// ```
    pub fn mount(&mut self, key: impl Into<Cow<'static, str>>, plugin: impl Plugin + 'static) -> &mut Self {
        self.plugins.insert(key.into(), Box::new(plugin));
        self
    }

    /// Uninstall a plugin
    ///
    /// ```
    /// let mut logger = logkit::Logger::def();
    /// logger.unmount("level");
    /// ```
    pub fn unmount(&mut self, key: &str) -> &mut Self {
        self.plugins.remove(key);
        self
    }

    /// Add a output target for records
    ///
    /// Multiple targets are supported, allowing you to output a single record to various places.
    ///
    /// ```
    /// let mut logger = logkit::Logger::new();
    /// logger.route("default", logkit::StdoutTarget);
    /// ```
    pub fn route(&mut self, key: impl Into<Cow<'static, str>>, target: impl Target + 'static) -> &mut Self {
        self.targets.insert(key.into(), Box::new(target));
        self
    }

    /// Remove a output target
    ///
    /// ```
    /// let mut logger = logkit::Logger::def();
    /// logger.unroute("default");
    /// ```
    pub fn unroute(&mut self, key: &str) -> &mut Self {
        self.targets.remove(key);
        self
    }

    /// Check if the log level is equal to or higher than the limit
    #[inline]
    pub fn allow(&self, level: Level) -> bool {
        level >= self.level
    }

    /// Create a new log record
    ///
    /// Internally, each log is represented by a record, which contains level information and
    /// a cached buffer. You can append numerous fields to a record. The println-like message is
    /// also treated as a normal field with the key named `msg`.
    ///
    /// ```
    /// let mut logger = logkit::Logger::new();
    /// logger.route("default", logkit::StdoutTarget);
    /// if let Some(mut record) = logger.spawn(logkit::LEVEL_TRACE) {
    ///     record.append("hello", "world");
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

        for (_, plugin) in &self.plugins {
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
    /// logger.route("default", logkit::StdoutTarget);
    /// if let Some(mut record) = logger.spawn(logkit::LEVEL_TRACE) {
    ///     record.append("msg", "this log will be directed to stdout");
    ///     logger.flush(record);
    /// }
    /// ```
    #[inline]
    pub fn flush(&self, mut record: Record) {
        for (_, plugin) in &self.plugins {
            if !plugin.post(&mut record) {
                self.reuse(record);
                return;
            }
        }

        record.finish();

        for (_, target) in &self.targets {
            target.write(record.buffer());
        }

        self.reuse(record);
    }

    /// Places the record back into the object pool for reuse.
    ///
    /// The `flush` method calls this function automatically, so typically you don't need to
    /// invoke it manually.
    #[inline]
    pub fn reuse(&self, record: Record) {
        let guard = self.records.lock();
        guard.borrow_mut().push(record);
    }
}