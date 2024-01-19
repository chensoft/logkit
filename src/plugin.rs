//! mod target
use super::define::*;
use super::record::*;

#[allow(unused_variables)]
pub trait Plugin: Sync + Send {
    #[must_use]
    fn pre(&self, record: &mut Record) -> bool {
        true
    }

    #[must_use]
    fn post(&self, record: &mut Record) -> bool {
        true
    }
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn pre(&self, record: &mut Record) -> bool {
        // todo
        // record.append("level", level_to_str(record.level()));
        true
    }
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
    fn pre(&self, record: &mut Record) -> bool {
        // todo
        // let now = chrono::Local::now();
        // record.append("time", now.to_rfc3339_opts(self.format, false)); // todo make faster
        true
    }
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

        // todo
        // "funcname".encode(buf);
        // buf.push(b':');
        // self.funcname.encode(buf);
        // buf.push(b',');
        //
        // "filename".encode(buf);
        // buf.push(b':');
        // self.filename.encode(buf);
        // buf.push(b',');
        //
        // "lineno".encode(buf);
        // buf.push(b':');
        // self.lineno.encode(buf);

        buf.push(b'}');
    }
}

pub struct StackPlugin {
    pub level: Level,
}

impl Plugin for StackPlugin {
    fn post(&self, record: &mut Record) -> bool {
        if record.level() != self.level || std::env::var("RUST_BACKTRACE").unwrap_or("0".to_string()) == "0" {
            return true;
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

        // todo
        // record.append("stack", frames);

        true
    }
}