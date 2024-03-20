//! Source represent source info

/// Source Info
///
/// Save the source info when the user calls the macros.
#[derive(Debug, Clone)]
pub struct Source {
    /// File name in which it was invoked
    pub file: &'static str,

    /// line number on which it was invoked
    pub line: u32,

    /// Column number at which it was invoked
    pub column: u32,
}

impl Source {
    /// Create a new source info
    ///
    /// ```
    /// let mut source = logkit::Source::new(file!(), line!(), column!());
    /// assert_eq!(source.file, "src/source.rs");
    /// assert!(source.line > 0);
    /// assert!(source.column > 0);
    /// ```
    #[inline]
    pub fn new(file: &'static str, line: u32, column: u32) -> Self {
        Self {file, line, column}
    }
}