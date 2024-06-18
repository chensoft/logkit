#![doc = include_str!("../README.md")]
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