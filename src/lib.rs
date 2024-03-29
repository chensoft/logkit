//! Logkit
//!
//! Logkit is a JSON-style structured logging library written in Rust, aiming to achieve high
//! performance and customizability. To reach these goals, We have created a fast encoding library
//! and added a flexible plugin system to it.
//!
//! ## Basic Syntax
//!
//! Five convenient macros are available for use: `trace`, `debug`, `info`, `warn`, and `error`.
//! These support the following log formats, and you can define custom macros if necessary.
//!
//! ```
//! #[macro_use] extern crate logkit;
//!
//! trace!(); // outputs just a linebreak
//! trace!("plain message");
//! trace!("println-like message {} {}!", "Hello", "World");
//! trace!(name = "Alice", age = 20); // outputs only fields, no message
//! trace!(name = "Alice", age = 20; "separate fields and messages with semicolon");
//! trace!(name = "Alice", age = 20; "println-like message {} {}! with fields", "Hello", "World");
//! ```
//!
//! ## Default Logger
//!
//! For convenience, we have defined a default logger that outputs messages to stderr.
//!
//! ```
//! #[macro_use] extern crate logkit;
//!
//! assert_eq!(logkit::default_logger().level(), logkit::LEVEL_TRACE);
//! trace!("hello, this is a trace log");
//! debug!("hello, this is a debug log");
//! ```
//!
//! ## Custom Logger
//!
//! ```
//! let mut logger = logkit::Logger::new(None);
//! logger.mount(logkit::LevelPlugin); // you can add your own plugin
//! logger.route(logkit::StderrTarget); // and add your custom target
//!
//! // replace the default logger
//! logkit::set_default_logger(logger);
//! // or use it directly like built-in macros
//! ```
//!
//! ## Custom Level
//!
//! There are five built-in log levels: `TRACE`, `DEBUG`, `INFO`, `WARN` and `ERROR`. You can define your
//! own levels, as the type is simply an alias for i32, not an enum.
//!
//! ```
//! pub const LEVEL_CUSTOM : logkit::Level = 10; // use any number distinct from the built-ins
//!
//! #[macro_export]
//! macro_rules! custom {
//!     ($($arg:tt)*) => {{
//!         logkit::record!(logkit::default_logger(), LEVEL_CUSTOM, $($arg)*)
//!     }};
//! }
//! 
//! custom!("this is a custom log level");
//! ```
//!
//! ## Custom Encoding
//! 
//! We support all scalar types and many std collections, if you want to encode your own type into
//! json, you can implement the Encode trait.
//! 
//! ```
//! pub struct CustomStruct {
//!     pub key1: i32,
//!     pub key2: bool,
//!     pub key3: String,
//! }
//!
//! impl logkit::Encode for CustomStruct {
//!     #[inline]
//!     fn encode(&self, buf: &mut Vec<u8>) {
//!         // format your struct into buf
//!         unimplemented!()
//!     }
//! }
//! ```
//! 
//! ## Logging Plugin
//!
//! Plugins, also known as middleware, add hooks for `pre` and `post` steps. When a logger spawns a
//! record, the `pre` method is called before any fields are added to it. When the record is ready
//! to flush, the `post` method is invoked before outputting to targets. You can add any fields
//! to the record. If you decide not to continue handling the record, simply return `false` in
//! `pre` or `post`. The record will not be processed further if `false` is returned.
//!
//! ```
//! #[macro_use] extern crate logkit;
//!
//! // custom plugin to add 'pid' to record
//! pub struct PidPlugin { pub pid: u32 }
//!
//! impl logkit::Plugin for PidPlugin {
//!     #[inline]
//!     fn post(&self, record: &mut logkit::Record) -> bool {
//!         record.append("pid", &self.pid);
//!         true
//!     }
//! }
//!
//! let mut logger = logkit::Logger::new(Some(&logkit::StderrTarget));
//! logger.mount(PidPlugin {pid: std::process::id()});
//! logkit::set_default_logger(logger);
//!
//! info!("you will see this log with a process id");
//! ```
//!
//! ```
//! #[macro_use] extern crate logkit;
//!
//! // custom plugin to filter all levels below 'info'
//! pub struct LimitPlugin;
//!
//! impl logkit::Plugin for LimitPlugin {
//!     #[inline]
//!     fn pre(&self, record: &mut logkit::Record) -> bool {
//!         record.level() >= logkit::LEVEL_INFO
//!     }
//! }
//!
//! let mut logger = logkit::Logger::new(Some(&logkit::StderrTarget));
//! logger.mount(LimitPlugin);
//! logkit::set_default_logger(logger);
//!
//! debug!("this log is ignored");
//! info!("you can see this log");
//! ```
//!
//! ## Output Target
//!
//! Upon completion, a record is routed to various targets, which define the methods of outputting
//! content. A record can be directed to multiple targets, and each target is simply required to
//! implement the `Target` trait.
//!
//! ```
//! #[macro_use] extern crate logkit;
//!
//! pub struct CustomTarget;
//!
//! impl logkit::Target for CustomTarget {
//!     #[inline]
//!     fn write(&self, buf: &[u8]) {
//!         use std::io::Write;
//!         let _ = std::io::stdout().write_all(buf);
//!     }
//! }
//!
//! let mut logger = logkit::Logger::new(Some(&logkit::StderrTarget));
//! logger.route(CustomTarget);
//! logkit::set_default_logger(logger);
//!
//! info!("record will be output to both stderr and stdout now");
//! ```
//!
//! **Happy Logging!**
#![warn(missing_docs)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::let_underscore_future)]

pub mod define;
pub mod logger;
pub mod macros;
pub mod plugin;
pub mod record;
pub mod source;
pub mod target;

#[doc(inline)]
pub use define::*;
#[doc(hidden)]
pub use logger::*;
#[doc(inline)]
pub use macros::*;
#[doc(hidden)]
pub use plugin::*;
#[doc(hidden)]
pub use record::*;
#[doc(hidden)]
pub use source::*;
#[doc(hidden)]
pub use target::*;