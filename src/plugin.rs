//! mod target
use super::define::*;
use super::encode::*;
use super::record::*;

pub trait Plugin: Sync + Send {
    fn pre(&self, record: &mut Record);

    fn post(&self, record: &mut Record);
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    #[inline]
    fn pre(&self, record: &mut Record) {
        record.append("level", level_to_str(record.level()));
    }

    #[inline]
    fn post(&self, _record: &mut Record) {}
}

pub struct TimePlugin {
    pub format: chrono::SecondsFormat,
}

impl TimePlugin {
    // e.g: 2024-01-03T11:01:00+08:00
    pub fn from_secs() -> Self {
        Self {format: chrono::SecondsFormat::Secs}
    }

    // e.g: 2024-01-03T11:01:00.123+08:00
    pub fn from_millis() -> Self {
        Self {format: chrono::SecondsFormat::Millis}
    }

    // e.g: 2024-01-03T11:01:00.123456+08:00
    pub fn from_micros() -> Self {
        Self {format: chrono::SecondsFormat::Micros}
    }

    // e.g: 2024-01-03T11:01:00.123456789+08:00
    pub fn from_nanos() -> Self {
        Self {format: chrono::SecondsFormat::Nanos}
    }
}

impl Plugin for TimePlugin {
    #[inline]
    fn pre(&self, record: &mut Record) {
        let now = chrono::Local::now();
        record.append("time", now.to_rfc3339_opts(self.format, false)); // todo make faster
    }

    #[inline]
    fn post(&self, _record: &mut Record) {}
}

#[derive(Debug, Default, Clone)]
pub struct StackFrame {
    pub funcname: String,
    pub filename: String,
    pub lineno: u32,
}

impl Encode for StackFrame {
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

pub struct StackPlugin {
    pub level: Level,
}

impl Plugin for StackPlugin {
    #[inline]
    fn pre(&self, _record: &mut Record) {}

    #[inline]
    fn post(&self, record: &mut Record) {
        if record.level() != self.level || std::env::var("RUST_BACKTRACE").unwrap_or("0".to_string()) == "0" {
            return;
        }

        let mut frames = vec![];

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

        record.append("stack", frames);
    }
}