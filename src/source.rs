//! Source represent source info

/// Source Info
///
/// Save the source info when the user calls the macros.
#[derive(Debug, Default, Clone)]
pub struct Source {
    /// File name in which it was invoked
    pub file: &'static str,

    /// line number on which it was invoked
    pub line: u32,

    /// Column number at which it was invoked
    pub column: u32,
}

/// Create a new source info
///
/// ```
/// let mut source = logkit::source!();
/// assert!(source.file == "src/source.rs" || source.file == "src\\source.rs");
/// assert!(source.line > 0);
/// assert!(source.column > 0);
/// ```
#[macro_export]
macro_rules! source {
    () => {{
        $crate::Source { file: file!(), line: line!(), column: column!() }
    }};
}