//! mod target
use super::define::*;

pub trait Target: Sync + Send {
    fn write(&self, buf: &[u8]);
}

pub struct StdoutTarget;

impl Target for StdoutTarget {
    fn write(&self, buf: &[u8]) {
        let _ = std::io::stdout().write_all(buf);
    }
}

pub struct StderrTarget;

impl Target for StderrTarget {
    fn write(&self, buf: &[u8]) {
        let _ = std::io::stderr().write_all(buf);
    }
}

pub struct FileTarget {}

impl Target for FileTarget {
    fn write(&self, _buf: &[u8]) {
        todo!()
    }
}