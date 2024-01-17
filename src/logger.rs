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

lazy_static! {
    static ref DEF_LOGGER: RwLock<Logger> = RwLock::new({
        let mut obj = Logger::new();
        obj.mount(Box::new(LevelPlugin));
        obj.mount(Box::new(TimePlugin));
        obj.mount(Box::new(StackPlugin {level: LEVEL_ERROR}));
        obj.route(Box::new(StdoutTarget));
        obj
    });
}

impl Logger {
    pub fn def() -> &'static RwLock<Logger> {
        &DEF_LOGGER
    }

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

    #[inline]
    pub fn allow(&self, level: Level) -> bool {
        level >= self.level
    }

    #[inline]
    pub fn spawn(&self, level: Level) -> Record {
        Record::new(level, self.alloc)
    }

    #[inline]
    pub fn write(&self, mut record: Record) { // todo reuse record
        for target in &self.targets {
            record.flush(target.as_ref());
        }
    }
}