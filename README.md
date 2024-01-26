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
| empty_log         | [16.576 ns 16.598 ns 16.622 ns] |
| level_off         | [1.1781 ns 1.1791 ns 1.1803 ns] |
| msg_only          | [53.629 ns 53.742 ns 53.868 ns] |
| msg_format        | [53.707 ns 53.790 ns 53.888 ns] |
| fields_only       | [90.106 ns 90.141 ns 90.175 ns] |
| fields_msg        | [136.68 ns 136.72 ns 136.76 ns] |
| fields_msg_format | [138.01 ns 138.62 ns 139.37 ns] |
| fields_ten_fields | [397.27 ns 398.05 ns 399.21 ns] |

- AWS c5.2xlarge, 8C 16G, Ubuntu 22.04

| Name              |              Time               |
|:------------------|:-------------------------------:|
| empty_log         | [46.816 ns 46.821 ns 46.827 ns] |
| level_off         | [2.4240 ns 2.4242 ns 2.4244 ns] |
| msg_only          | [107.52 ns 107.54 ns 107.55 ns] |
| msg_format        | [107.49 ns 107.52 ns 107.55 ns] |
| fields_only       | [186.12 ns 186.17 ns 186.22 ns] |
| fields_msg        | [257.24 ns 257.29 ns 257.35 ns] |
| fields_msg_format | [257.20 ns 257.26 ns 257.31 ns] |
| fields_ten_fields | [709.65 ns 709.72 ns 709.78 ns] |

## Documentation

The documentation is [available here](https://docs.rs/logkit).

## License

This software is released under the [MIT License](https://github.com/chensoft/logkit?tab=MIT-1-ov-file).