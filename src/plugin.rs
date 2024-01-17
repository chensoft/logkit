//! mod target
use super::define::*;

pub trait Plugin: Sync + Send {
    // todo &mut Record to plugin
    fn pre(&self);
    
    fn post(&self);
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn pre(&self) {
        todo!()
    }

    #[inline] // todo tell rust do not generate
    fn post(&self) {
        todo!()
    }
}

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn pre(&self) {
        todo!()
    }

    fn post(&self) {
        todo!()
    }
}

pub struct StackPlugin {
    pub level: Level,
}

impl Plugin for StackPlugin {
    fn pre(&self) {
        todo!()
    }

    fn post(&self) {
        todo!()
    }
}