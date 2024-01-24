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
//! For ease of use, the crate defines a default logger with some predefined behaviors, such as
//! printing a level, an RFC3339 milliseconds datetime, capturing a stack trace for the ERROR
//! level, and outputting messages to stdout by default.
//!
//! To access or modify the default logger, you can use functions as follows:
//!
//! ```
//! #[macro_use] extern crate logkit;
//!
//! debug!("the current log level is {}", logkit::default_logger().read().level);
//!
//! logkit::default_logger().write().level = logkit::LEVEL_INFO;
//! debug!("debug logs are now hidden");
//! info!("only logs with a level of 'info' or higher will be visible");
//! ```
//!
//! ## Custom Logger
//!
//! The default logger is highly customizable, you can unmount all its predefined plugins and unroute
//! all its targets, making it rarely necessary to create your own logger. However, if you need to
//! create a custom logger, you can follow the code example below:
//!
//! ```
//! let mut logger = logkit::Logger::new();
//! logger.mount("level", logkit::LevelPlugin); // you can define your own plugin
//! logger.route("default", logkit::StderrTarget); // and define your custom target
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
//!         record.append("pid", self.pid);
//!         true
//!     }
//! }
//!
//! logkit::default_logger().write().mount("pid", PidPlugin {pid: std::process::id()});
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
//! logkit::default_logger().write().mount("limit", LimitPlugin);
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
//! pub struct StderrTarget;
//!
//! impl logkit::Target for StderrTarget {
//!     #[inline]
//!     fn write(&self, buf: &[u8]) {
//!         use std::io::Write;
//!         let _ = std::io::stderr().write_all(buf);
//!     }
//! }
//!
//! logkit::default_logger().write().route("stderr", StderrTarget);
//! info!("record will be output to both stdout and stderr now");
//! ```
//!
//! **Happy Logging!**
#[macro_use] extern crate lazy_static;

pub mod define;
pub mod logger;
pub mod macros;
pub mod plugin;
pub mod record;
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
pub use target::*;