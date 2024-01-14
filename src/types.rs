// pub(crate) use std::collections::BTreeMap;

pub type Level = i32;

pub const LEVEL_TRACE : Level = 0;
pub const LEVEL_DEBUG : Level = 1;
pub const LEVEL_INFO  : Level = 2;
pub const LEVEL_WARN  : Level = 3;
pub const LEVEL_ERROR : Level = 4;
pub const LEVEL_OFF   : Level = i32::MAX;