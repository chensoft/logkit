Logkit
==========================

Super fast, structured, scalable logging library for Rust

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][license-badge]][license-url]
[![Documentation][document-badge]][document-url]
[![Build Status][linux-badge]][linux-url]
[![Build Status][macos-badge]][macos-url]
[![Build Status][windows-badge]][windows-url]

[crates-badge]: https://img.shields.io/crates/v/logkit.svg
[crates-url]: https://crates.io/crates/logkit
[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[license-url]: https://github.com/chensoft/logkit?tab=MIT-1-ov-file
[document-badge]: https://docs.rs/logkit/badge.svg
[document-url]: https://docs.rs/logkit
[linux-badge]: https://github.com/chensoft/logkit/actions/workflows/linux.yml/badge.svg
[linux-url]: https://github.com/chensoft/logkit/actions/workflows/linux.yml
[macos-badge]: https://github.com/chensoft/logkit/actions/workflows/macos.yml/badge.svg
[macos-url]: https://github.com/chensoft/logkit/actions/workflows/macos.yml
[windows-badge]: https://github.com/chensoft/logkit/actions/workflows/windows.yml/badge.svg
[windows-url]: https://github.com/chensoft/logkit/actions/workflows/windows.yml

## Hello World

```rust
#[macro_use] extern crate logkit;

fn main() {
    let mut logger = logkit::Logger::new(Some(&logkit::StdoutTarget));
    logger.mount(logkit::LevelPlugin);
    logger.mount(logkit::TimePlugin::from_millis());
    logkit::set_default_logger(logger);

    trace!("hello, this is a trace log");
    debug!("hello, this is a debug log");
    info!(version = "0.1.0", commit = "3291cc60"; "this is a log with two string fields");
    warn!(address = "127.0.0.1", port = 3000; "this is a log with a string and a numeric field");
    error!("this is a log with a 'println' style string {}:{}", "127.0.0.1", 3000.0);
}
```

Output sample:

```json
{"level":"trace","time":"2024-01-26T20:33:01.841+08:00","msg":"hello, this is a trace log"}
{"level":"debug","time":"2024-01-26T20:33:01.841+08:00","msg":"hello, this is a debug log"}
{"level":"info","time":"2024-01-26T20:33:01.841+08:00","msg":"this is a log with two string fields","version":"0.1.0","commit":"3291cc60"}
{"level":"warn","time":"2024-01-26T20:33:01.841+08:00","msg":"this is a log with a string and a numeric field","address":"127.0.0.1","port":3000}
{"level":"error","time":"2024-01-26T20:33:01.841+08:00","msg":"this is a log with a 'println' style string 127.0.0.1:3000"}
```

## Basic Syntax

Five convenient macros are available for use: `trace`, `debug`, `info`, `warn`, and `error`.
These support the following log formats, and you can define custom macros if necessary.

```rust
#[macro_use] extern crate logkit;

trace!(); // outputs just a linebreak
trace!("plain message");
trace!("println-like message {} {}!", "Hello", "World");
trace!(name = "Alice", age = 20); // outputs only fields, no message
trace!(name = "Alice", age = 20; "separate fields and messages with semicolon");
trace!(name = "Alice", age = 20; "println-like message {} {}! with fields", "Hello", "World");
```

## Default Logger

For convenience, we have defined a default logger that outputs messages to stderr.

```rust
#[macro_use] extern crate logkit;

assert_eq!(logkit::default_logger().level(), logkit::LEVEL_TRACE);
trace!("hello, this is a trace log");
debug!("hello, this is a debug log");
```

## Custom Logger

```rust
fn main() {
    let mut logger = logkit::Logger::new(None);
    logger.mount(logkit::LevelPlugin); // you can add your own plugin
    logger.route(logkit::StderrTarget); // and add your custom target

    // replace the default logger
    logkit::set_default_logger(logger);
    // or use it directly like built-in macros
}
```

## Custom Level

There are five built-in log levels: `TRACE`, `DEBUG`, `INFO`, `WARN` and `ERROR`. You can define your
own levels, as the type is simply an alias for i32, not an enum.

```rust
pub const LEVEL_CUSTOM : logkit::Level = 10; // use any number distinct from the built-ins

#[macro_export]
macro_rules! custom {
    ($($arg:tt)*) => {{
        logkit::record!(logkit::default_logger(), LEVEL_CUSTOM, $($arg)*)
    }};
}

custom!("this is a custom log level");
```

## Custom Encoding

We support all scalar types and many std collections, if you want to encode your own type into
json, you can implement the Encode trait.

```rust
pub struct CustomStruct {
    pub key1: i32,
    pub key2: bool,
    pub key3: String,
}

impl logkit::Encode for CustomStruct {
    #[inline]
    fn encode(&self, buf: &mut Vec<u8>) {
        // format your struct into buf
        unimplemented!()
    }
}
```

## Logging Plugin

Plugins, also known as middleware, add hooks for `pre` and `post` steps. When a logger spawns a
record, the `pre` method is called before any fields are added to it. When the record is ready
to flush, the `post` method is invoked before outputting to targets. You can add any fields
to the record. If you decide not to continue handling the record, simply return `false` in
`pre` or `post`. The record will not be processed further if `false` is returned.

```rust
#[macro_use] extern crate logkit;

// custom plugin to add 'pid' to record
pub struct PidPlugin { pub pid: u32 }

impl logkit::Plugin for PidPlugin {
    #[inline]
    fn post(&self, record: &mut logkit::Record) -> bool {
        record.append("pid", &self.pid);
        true
    }
}

fn main() {
    let mut logger = logkit::Logger::new(Some(&logkit::StderrTarget));
    logger.mount(PidPlugin { pid: std::process::id() });
    logkit::set_default_logger(logger);

    info!("you will see this log with a process id");
}
```

```rust
#[macro_use] extern crate logkit;

// custom plugin to filter all levels below 'info'
pub struct LimitPlugin;

impl logkit::Plugin for LimitPlugin {
    #[inline]
    fn pre(&self, record: &mut logkit::Record) -> bool {
        record.level() >= logkit::LEVEL_INFO
    }
}

fn main() {
    let mut logger = logkit::Logger::new(Some(&logkit::StderrTarget));
    logger.mount(LimitPlugin);
    logkit::set_default_logger(logger);

    debug!("this log is ignored");
    info!("you can see this log");
}
```

## Output Target

Upon completion, a record is routed to various targets, which define the methods of outputting
content. A record can be directed to multiple targets, and each target is simply required to
implement the `Target` trait.

```rust
#[macro_use] extern crate logkit;

pub struct CustomTarget;

impl logkit::Target for CustomTarget {
    #[inline]
    fn write(&self, buf: &[u8]) {
        use std::io::Write;
        let _ = std::io::stdout().write_all(buf);
    }
}

fn main() {
    let mut logger = logkit::Logger::new(Some(&logkit::StderrTarget));
    logger.route(CustomTarget);
    logkit::set_default_logger(logger);

    info!("record will be output to both stderr and stdout now");
}
```

## Benchmark

- MacBook Air, Apple M2 24G, Sonoma 14.2.1

| Name              |              Time               |
|:------------------|:-------------------------------:|
| empty_log         | [22.526 ns 22.541 ns 22.560 ns] |
| level_off         | [1.6941 ns 1.6989 ns 1.7050 ns] |
| msg_only          | [63.166 ns 63.172 ns 63.177 ns] |
| msg_format        | [63.238 ns 63.373 ns 63.548 ns] |
| fields_only       | [96.944 ns 96.974 ns 97.005 ns] |
| fields_msg        | [147.03 ns 147.26 ns 147.56 ns] |
| fields_msg_format | [146.44 ns 146.51 ns 146.58 ns] |
| fields_ten_fields | [395.31 ns 395.35 ns 395.40 ns] |

- AWS c5.2xlarge, 8C 16G, Ubuntu 22.04

| Name              |              Time               |
|:------------------|:-------------------------------:|
| empty_log         | [50.761 ns 50.764 ns 50.768 ns] |
| level_off         | [4.1800 ns 4.1804 ns 4.1810 ns] |
| msg_only          | [121.12 ns 121.14 ns 121.16 ns] |
| msg_format        | [121.18 ns 121.20 ns 121.23 ns] |
| fields_only       | [177.70 ns 177.74 ns 177.77 ns] |
| fields_msg        | [264.25 ns 264.33 ns 264.45 ns] |
| fields_msg_format | [261.80 ns 261.89 ns 261.98 ns] |
| fields_ten_fields | [654.11 ns 654.31 ns 654.51 ns] |