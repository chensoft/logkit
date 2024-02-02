//! Target trait and built-in output targets
use super::define::*;

/// The Target Trait
///
/// After completion, a record is directed to targets, whose purpose is to output the record's
/// content to various locations. A single record can be associated with multiple targets.
pub trait Target: Sync + Send + 'static {
    fn write(&self, buf: &[u8]);
}

/// To store arbitrary targets
pub trait AnyTarget: Target + Any {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Target + Any> AnyTarget for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Write to stdout
///
/// ```
/// let logger = logkit::Logger::new(Some(&logkit::StdoutTarget));
/// logkit::set_default_logger(logger);
/// ```
pub struct StdoutTarget;

impl Target for StdoutTarget {
    #[inline]
    fn write(&self, buf: &[u8]) {
        let _ = std::io::stdout().write_all(buf);
    }
}

/// Write to stderr
///
/// ```
/// let logger = logkit::Logger::new(Some(&logkit::StderrTarget));
/// logkit::set_default_logger(logger);
/// ```
pub struct StderrTarget;

impl Target for StderrTarget {
    #[inline]
    fn write(&self, buf: &[u8]) {
        let _ = std::io::stderr().write_all(buf);
    }
}

/// Write to a file
///
/// ```
/// fn main() -> anyhow::Result<()> {
///     let mut sample = std::env::temp_dir();
///     sample.push("sample.log");
/// 
///     let mut logger = logkit::Logger::new(None);
///     logger.route(logkit::FileTarget::new(sample)?);
///     logkit::set_default_logger(logger);
///
///     Ok(())
/// }
/// ```
pub struct FileTarget {
    pub file: Mutex<RefCell<std::fs::File>>,
}

impl FileTarget {
    pub fn new(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        Ok(Self {file: Mutex::new(RefCell::new(std::fs::OpenOptions::new().create(true).append(true).open(path)?))})
    }
}

impl Target for FileTarget {
    #[inline]
    fn write(&self, buf: &[u8]) {
        let file = self.file.lock();
        let _ = file.borrow_mut().write_all(buf);
    }
}