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

## Documentation

The documentation is [available here](https://docs.rs/logkit).

## License

This software is released under the [MIT License](https://github.com/chensoft/logkit?tab=MIT-1-ov-file).