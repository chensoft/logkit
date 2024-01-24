//! Log levels and Encode trait
pub(crate) use std::io::Write;
pub(crate) use std::path::Path;
pub(crate) use std::borrow::Cow;
pub(crate) use std::cell::RefCell;
pub(crate) use indexmap::IndexMap;
pub(crate) use parking_lot::RwLock;
pub(crate) use parking_lot::ReentrantMutex;

/// Log Level
/// 
/// This type is simply an alias for i32. It was not implemented as an enum to allow for extension.
/// 
/// ```
/// pub const LEVEL_CUSTOM : logkit::Level = 10; // use any number distinct from the built-ins
///
/// #[macro_export]
/// macro_rules! custom {
///     ($($arg:tt)*) => {{
///         logkit::record!(logkit::default_logger(), LEVEL_CUSTOM, $($arg)*)
///     }};
/// }
///
/// custom!("this is a custom log level");
/// ```
pub type Level = i32;

pub const LEVEL_TRACE : Level = 0;
pub const LEVEL_DEBUG : Level = 1;
pub const LEVEL_INFO  : Level = 2;
pub const LEVEL_WARN  : Level = 3;
pub const LEVEL_ERROR : Level = 4;
pub const LEVEL_OFF   : Level = i32::MAX;

/// Level to string
#[inline]
pub fn level_to_str(level: Level) -> Option<&'static str> {
    match level {
        LEVEL_TRACE => Some("trace"),
        LEVEL_DEBUG => Some("debug"),
        LEVEL_INFO => Some("info"),
        LEVEL_WARN => Some("warn"),
        LEVEL_ERROR => Some("error"),
        _ => None,
    }
}

/// String to level
#[inline]
pub fn str_to_level(level: &str) -> Level {
    match level {
        "trace" => LEVEL_TRACE,
        "debug" => LEVEL_DEBUG,
        "info" => LEVEL_INFO,
        "warn" => LEVEL_WARN,
        "error" => LEVEL_ERROR,
        _ => LEVEL_OFF,
    }
}

/// Encode Trait
///
/// This trait is used for formatting a field's value. Encoding support has already been added for
/// all scalar types and many standard collections. If you wish to format your own type, just
/// implement this trait.
///
/// ```
/// pub struct CustomStruct {
///     pub key1: i32,
///     pub key2: bool,
///     pub key3: String,
/// }
///
/// impl logkit::Encode for CustomStruct {
///     #[inline]
///     fn encode(&self, buf: &mut Vec<u8>) {
///         // format your struct into buf
///         unimplemented!()
///     }
/// }
/// ```
pub use encoder::json::Encode;