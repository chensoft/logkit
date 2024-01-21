//! mod logger
use super::define::*;
use super::record::*;
use super::plugin::*;
use super::target::*;

pub struct Logger {
    pub level: Level, // logging level limit
    pub alloc: usize, // record init capacity

    plugins: Vec<Box<dyn Plugin>>, // middlewares
    targets: Vec<Box<dyn Target>>, // output targets
}

// todo use Allocator to new record
impl Logger {
    pub fn new() -> Self {
        let mut obj = Self {level: LEVEL_TRACE, alloc: 512, plugins: vec![], targets: vec![]};

        if let Ok(level) = std::env::var("RUST_LOG") {
            obj.level = match level.parse::<Level>() {
                Ok(val) => val,
                Err(_) => str_to_level(level.to_lowercase().as_str()),
            };
        }

        obj
    }

    pub fn mount(&mut self, plugin: Box<dyn Plugin>) -> &mut Self {
        self.plugins.push(plugin);
        self
    }

    pub fn route(&mut self, target: Box<dyn Target>) -> &mut Self {
        self.targets.push(target);
        self
    }

    pub fn allow(&self, level: Level) -> bool {
        level >= self.level
    }

    pub fn spawn(&self, level: Level) -> Option<Record> {
        if !self.allow(level) {
            return None;
        }

        let mut record = Record::new(level, self.alloc);

        for plugin in &self.plugins {
            if !plugin.pre(&mut record) {
                return None;
            }
        }

        Some(record)
    }

    pub fn write(&self, mut record: Record) {
        for plugin in &self.plugins {
            if !plugin.post(&mut record) {
                return;
            }
        }

        record.finish();

        for target in &self.targets {
            target.write(record.buffer());
        }
    }
}