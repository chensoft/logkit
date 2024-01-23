//! The core logger struct to manage logging
use super::define::*;
use super::record::*;
use super::plugin::*;
use super::target::*;

pub struct Logger {
    pub level: Level, // logging level limit
    pub alloc: usize, // record init capacity

    records: ReentrantMutex<RefCell<Vec<Record>>>, // records pool
    plugins: IndexMap<Cow<'static, str>, Box<dyn Plugin>>,    // middlewares
    targets: IndexMap<Cow<'static, str>, Box<dyn Target>>,    // output targets
}

impl Logger {
    pub fn new() -> Self {
        let mut obj = Self {
            level: LEVEL_TRACE,
            alloc: 512,
            records: ReentrantMutex::new(RefCell::new(vec![])),
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

    pub fn def() -> Self {
        let mut obj = Logger::new();
        obj.mount("level", Box::new(LevelPlugin));
        obj.mount("time", Box::new(TimePlugin::from_millis()));
        obj.mount("stack", Box::new(StackPlugin {level: LEVEL_ERROR}));
        obj.route("stdout", Box::new(StdoutTarget));
        obj
    }

    pub fn mount(&mut self, key: impl Into<Cow<'static, str>>, plugin: Box<dyn Plugin>) -> &mut Self {
        self.plugins.insert(key.into(), plugin);
        self
    }

    pub fn unmount(&mut self, key: &str) -> &mut Self {
        self.plugins.remove(key);
        self
    }

    pub fn route(&mut self, key: impl Into<Cow<'static, str>>, target: Box<dyn Target>) -> &mut Self {
        self.targets.insert(key.into(), target);
        self
    }

    pub fn unroute(&mut self, key: &str) -> &mut Self {
        self.targets.remove(key);
        self
    }

    #[inline]
    pub fn allow(&self, level: Level) -> bool {
        level >= self.level
    }

    #[inline]
    pub fn spawn(&self, level: Level) -> Option<Record> {
        if !self.allow(level) {
            return None;
        }

        let mut record = {
            let guard = self.records.lock();
            let mut array = guard.borrow_mut();
            match array.pop() {
                None => Record::default(),
                Some(val) => val,
            }
        };

        record.reset(level, self.alloc);

        for (_, plugin) in &self.plugins {
            if !plugin.pre(&mut record) {
                self.reuse(record);
                return None;
            }
        }

        Some(record)
    }

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

    #[inline]
    pub fn reuse(&self, record: Record) {
        {
            let guard = self.records.lock();
            let mut array = guard.borrow_mut();
            array.push(record);
        }
    }
}