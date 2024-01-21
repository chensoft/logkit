//! mod target
use super::define::*;

pub trait Target: Sync + Send {
    fn write(&self, buf: &[u8]);
}

pub struct StdoutTarget;

impl Target for StdoutTarget {
    #[inline]
    fn write(&self, buf: &[u8]) {
        let _ = std::io::stdout().write_all(buf);
    }
}

pub struct StderrTarget;

impl Target for StderrTarget {
    #[inline]
    fn write(&self, buf: &[u8]) {
        let _ = std::io::stderr().write_all(buf);
    }
}

pub struct FileTarget {
    pub file: ReentrantMutex<RefCell<std::fs::File>>,
}

impl FileTarget {
    pub fn new(path: &str) -> anyhow::Result<Self> {
        Ok(Self {file: ReentrantMutex::new(RefCell::new(std::fs::OpenOptions::new().create(true).append(true).open(path)?))})
    }
}

impl Target for FileTarget {
    #[inline]
    fn write(&self, buf: &[u8]) {
        {
            let file = self.file.lock();
            let _ = file.borrow_mut().write_all(buf);
        }
    }
}