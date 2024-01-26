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
    let logger = logkit::Logger::from_def();
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
{"level":"trace","time":"2024-01-25T23:14:24.747+08:00","msg":"hello, this is a trace log"}
{"level":"debug","time":"2024-01-25T23:14:24.747+08:00","msg":"hello, this is a debug log"}
{"level":"info","time":"2024-01-25T23:14:24.747+08:00","msg":"this is a log with two string fields","version":"0.1.0","commit":"3291cc60"}
{"level":"warn","time":"2024-01-25T23:14:24.747+08:00","msg":"this is a log with a string and a numeric field","address":"127.0.0.1","port":3000}
{"level":"error","time":"2024-01-25T23:14:24.747+08:00","msg":"this is a log with a 'println' style string 127.0.0.1:3000","stack":[{"funcname":"hello_world::main::h62df54cfc9622e64","filename":"/logkit/examples/hello_world.rs","lineno":11}]}
```

## Benchmark

- MacBook Air, Apple M2 24G, macOS 14.2.1

| Name              |              Time               |
|:------------------|:-------------------------------:|
| empty_log         | [27.267 ns 27.301 ns 27.338 ns] |
| level_off         | [6.4178 ns 6.4320 ns 6.4461 ns] |
| msg_only          | [61.319 ns 61.368 ns 61.420 ns] |
| msg_format        | [61.276 ns 61.333 ns 61.394 ns] |
| fields_only       | [95.099 ns 95.215 ns 95.345 ns] |
| fields_msg        | [144.15 ns 144.27 ns 144.41 ns] |
| fields_msg_format | [144.08 ns 144.20 ns 144.33 ns] |
| fields_ten_fields | [393.94 ns 396.83 ns 401.30 ns] |

- AWS c5.2xlarge, 8C 16G, Ubuntu 22.04

| Name              |              Time               |
|:------------------|:-------------------------------:|
| empty_log         | [73.345 ns 73.382 ns 73.419 ns] |
| level_off         | [16.151 ns 16.152 ns 16.153 ns] |
| msg_only          | [150.31 ns 150.33 ns 150.35 ns] |
| msg_format        | [146.70 ns 146.71 ns 146.73 ns] |
| fields_only       | [228.48 ns 228.51 ns 228.55 ns] |
| fields_msg        | [314.71 ns 314.75 ns 314.79 ns] |
| fields_msg_format | [314.65 ns 314.69 ns 314.73 ns] |
| fields_ten_fields | [728.83 ns 729.04 ns 729.31 ns] |

## Documentation

The documentation is [available here](https://docs.rs/logkit).

## License

This software is released under the [MIT License](https://github.com/chensoft/logkit?tab=MIT-1-ov-file).