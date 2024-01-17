//! mod target
pub trait Target: Send + Sync {}

pub struct ConsoleTarget;

pub struct FileTarget;