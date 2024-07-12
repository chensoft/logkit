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
pub trait Plugin: AnyPlugin + Send + Sync + 'static {
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

/// Any Support
pub trait AnyPlugin: Any {
    /// Treat object as any
    fn as_any(&self) -> &dyn Any;
}

impl<T: Any> AnyPlugin for T {
    #[inline]
    fn as_any(&self) -> &dyn Any {
        self
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

/// Add a datetime string to a record
pub struct TimePlugin {
    /// local or utc
    pub local: bool,

    /// time format
    pub format: time::format_description::OwnedFormatItem,
}

impl TimePlugin {
    /// Second-level precision
    ///
    /// ```json,no_run
    /// {"time":"2024-01-03T11:01:00+08:00"}
    /// ```
    pub fn from_secs() -> Self {
        Self { local: true, format: time::format_description::parse_owned::<2>("[year]-[month]-[day]T[hour]:[minute]:[second][offset_hour sign:mandatory]:[offset_minute]").unwrap_or_else(|_| unreachable!()) }
    }

    /// Millisecond-level precision
    ///
    /// ```json,no_run
    /// {"time":"2024-01-03T11:01:00.123+08:00"}
    /// ```
    pub fn from_millis() -> Self {
        Self { local: true, format: time::format_description::parse_owned::<2>("[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3][offset_hour sign:mandatory]:[offset_minute]").unwrap_or_else(|_| unreachable!()) }
    }

    /// Microsecond-level precision
    ///
    /// ```json,no_run
    /// {"time":"2024-01-03T11:01:00.123456+08:00"}
    /// ```
    pub fn from_micros() -> Self {
        Self { local: true, format: time::format_description::parse_owned::<2>("[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:6][offset_hour sign:mandatory]:[offset_minute]").unwrap_or_else(|_| unreachable!()) }
    }

    /// Nanosecond-level precision
    ///
    /// ```json,no_run
    /// {"time":"2024-01-03T11:01:00.123456789+08:00"}
    /// ```
    pub fn from_nanos() -> Self {
        Self { local: true, format: time::format_description::parse_owned::<2>("[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:9][offset_hour sign:mandatory]:[offset_minute]").unwrap_or_else(|_| unreachable!()) }
    }

    /// Use UTC for formatting
    ///
    /// ```json,no_run
    /// {"time":"2024-01-03T03:01:00+00:00"}
    /// ```
    pub fn use_utc(mut self) -> Self {
        self.local = false;
        self
    }
}

impl Plugin for TimePlugin {
    #[inline]
    fn pre(&self, record: &mut Record) -> bool {
        let now = match self.local {
            true => time::OffsetDateTime::now_local().unwrap_or(time::OffsetDateTime::now_utc()),
            false => time::OffsetDateTime::now_utc(),
        };

        record.append("time", &now.format(&self.format).unwrap_or_default());
        true
    }
}

/// Add source info to a record
///
/// ```json,no_run
/// {"src":"examples/hello_world.rs:9"}
/// ```
pub struct SourcePlugin {
    /// logs equal or greater than this level will include source info
    pub level: Level,
}

impl SourcePlugin {
    /// Create default plugin
    pub fn new() -> Self {
        Self { level: LEVEL_TRACE }
    }

    /// Create from level
    pub fn level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }
}

impl Plugin for SourcePlugin {
    #[inline]
    fn post(&self, record: &mut Record) -> bool {
        if record.level() >= self.level {
            record.append("src", &format!("{}:{}", record.source().file, record.source().line));
        }
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