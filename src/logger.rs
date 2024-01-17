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
        // todo add level, time hook
        // todo add stack for error
        // todo add target
        Logger::new()
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

    #[inline]
    pub fn allow(&self, level: Level) -> bool {
        level >= self.level
    }

    #[inline]
    pub fn spawn(&self, level: Level) -> Record {
        // todo &mut Record to plugin
        Record::new(level, self.alloc)
    }

    pub fn mount(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }
    
    pub fn route(&mut self, target: Box<dyn Target>) {
        self.targets.push(target);
    }
}