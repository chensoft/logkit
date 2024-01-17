//! mod target
pub trait Plugin: Send + Sync {
    fn pre(&self);
    
    fn post(&self);
}

pub struct LevelPlugin;

pub struct TimePlugin;