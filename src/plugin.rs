//! Plugin trait and some built-in plugins
use super::define::*;
use super::record::*;

/// The Plugin Trait
///
/// A plugin can be used to customize a record. You can append additional fields to a record before
/// or after the `msg` field.
///
/// You can terminate the log processing in advance, simply return `false` in `pre` or `post`.
#[allow(unused_variables)]
pub trait Plugin: Any + Sync + Send + 'static {
    /// Invoked before the `msg` field is appended to a record
    #[inline]
    #[must_use]
    fn pre(&self, record: &mut Record) -> bool {
        true
    }

    /// Invoked after the `msg` field is appended to a record
    #[inline]
    #[must_use]
    fn post(&self, record: &mut Record) -> bool {
        true
    }
}

/// Add a level string to a record
///
/// ```json,no_run
/// {"level":"info"}
/// ```
pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    #[inline]
    fn pre(&self, record: &mut Record) -> bool {
        match level_to_str(record.level()) {
            None => record.append("level", &record.level().to_string()),
            Some(level) => record.append("level", &level),
        };

        true
    }
}

/// Add a rfc3339 datetime string to a record
pub struct TimePlugin {
    /// time format
    pub format: chrono::SecondsFormat,
}

impl TimePlugin {
    /// Second-level precision
    ///
    /// ```json,no_run
    /// {"time":"2024-01-03T11:01:00+08:00"}
    /// ```
    pub fn from_secs() -> Self {
        Self {format: chrono::SecondsFormat::Secs}
    }

    /// Millisecond-level precision
    ///
    /// ```json,no_run
    /// {"time":"2024-01-03T11:01:00.123+08:00"}
    /// ```
    pub fn from_millis() -> Self {
        Self {format: chrono::SecondsFormat::Millis}
    }

    /// Microsecond-level precision
    ///
    /// ```json,no_run
    /// {"time":"2024-01-03T11:01:00.123456+08:00"}
    /// ```
    pub fn from_micros() -> Self {
        Self {format: chrono::SecondsFormat::Micros}
    }

    /// Nanosecond-level precision
    ///
    /// ```json,no_run
    /// {"time":"2024-01-03T11:01:00.123456789+08:00"}
    /// ```
    pub fn from_nanos() -> Self {
        Self {format: chrono::SecondsFormat::Nanos}
    }
}

impl Plugin for TimePlugin {
    #[inline]
    fn pre(&self, record: &mut Record) -> bool {
        let now = chrono::Local::now();
        record.append("time", &now.to_rfc3339_opts(self.format, false));
        true
    }
}

/// Represent a stack trace frame
#[derive(Debug, Default, Clone)]
pub struct StackFrame {
    /// function name
    pub funcname: String,

    /// file name
    pub filename: String,

    /// line number
    pub lineno: u32,
}

impl Encode for StackFrame {
    #[inline]
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.push(b'{');

        "funcname".encode(buf);
        buf.push(b':');
        self.funcname.encode(buf);
        buf.push(b',');

        "filename".encode(buf);
        buf.push(b':');
        self.filename.encode(buf);
        buf.push(b',');

        "lineno".encode(buf);
        buf.push(b':');
        self.lineno.encode(buf);

        buf.push(b'}');
    }
}

/// Add a stack trace to a record
///
/// Note that this plugin disregards frames internal to Rust and this crate.
///
/// ```json,no_run
/// {"stack":[{"funcname":"hello_world::main::h95297a3226de826e","filename":"/logkit/examples/hello_world.rs","lineno":9}]}
/// ```
pub struct StackPlugin {
    /// logs equal to this level will include a stack trace
    pub level: Level,
}

impl StackPlugin {
    /// Create from level
    pub fn from_level(level: Level) -> Self {
        Self {level}
    }
}

impl Plugin for StackPlugin {
    fn post(&self, record: &mut Record) -> bool {
        if record.level() != self.level {
            return true;
        }

        match std::env::var("RUST_BACKTRACE") {
            Ok(val) if val != "0" => {}
            _ => return true,
        }

        let mut frames = vec![];

        // todo pretty
        backtrace::trace(|frame| {
            backtrace::resolve_frame(frame, |symbol| {
                if let (Some(funcname), Some(filename), Some(lineno)) = (symbol.name(), symbol.filename(), symbol.lineno()) {
                    let funcname = funcname.to_string();
                    let filename = filename.to_string_lossy().to_string();

                    if filename.starts_with("/rustc/") ||
                        funcname.starts_with("backtrace::") ||
                        funcname.starts_with(concat!(env!("CARGO_PKG_NAME"), "::")) ||
                        funcname.starts_with(concat!("<", env!("CARGO_PKG_NAME"))) {
                        return;
                    }

                    frames.push(StackFrame {funcname, filename, lineno});
                }
            });

            true
        });

        record.append("stack", &frames);

        true
    }
}