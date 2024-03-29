//! Target trait and built-in output targets
use super::define::*;

/// The Target Trait
///
/// After completion, a record is directed to targets, whose purpose is to output the record's
/// content to various locations. A single record can be associated with multiple targets.
pub trait Target: AnyTarget + Send + Sync + 'static {
    /// Write logs from buf to target
    fn write(&self, buf: &[u8]);
}

/// Any Support
pub trait AnyTarget: Any {
    /// Treat object as any
    fn as_any(&self) -> &dyn Any;
}

impl<T: Any> AnyTarget for T {
    #[inline]
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
    /// file handle
    pub file: Mutex<std::fs::File>,
}

impl FileTarget {
    /// Create a FileTarget with a path
    pub fn new(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        if let Some(dir) = path.as_ref().parent() {
            std::fs::create_dir_all(dir)?;
        }

        Ok(Self {file: Mutex::new(std::fs::OpenOptions::new().create(true).append(true).open(path)?)})
    }
}

impl Target for FileTarget {
    #[inline]
    fn write(&self, buf: &[u8]) {
        match self.file.lock() {
            Ok(mut obj) => match obj.write_all(buf) {
                Ok(_) => {}
                Err(err) => { eprintln!("Error: {}", err); }
            }
            Err(err) => { eprintln!("Error: {}", err); }
        };
    }
}