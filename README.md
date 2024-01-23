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
[license-url]: https://github.com/chensoft/logkit/blob/master/LICENSE
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
    // logging with the default logger
    trace!("hello, this is a trace log");
    debug!("hello, this is a debug log");
    info!(version = "0.1.0", commit = "3291cc60"; "this is a log with two string fields");
    warn!(address = "127.0.0.1", port = 3000; "this is a log with a string and a numeric field");
    error!("this is a log with a 'println' style string {}:{}", "127.0.0.1", 3000.0);

    // set default logger's log level
    logkit::default_logger_mut().level = logkit::LEVEL_INFO;

    debug!("you can't see this log because the level is below 'info'");
    info!("only logs with a level equal to or higher than 'info' will be printed");
    error!("you can see this error log with stack trace");

    // remove logger's stack plugin
    logkit::default_logger_mut().unmount("stack");

    error!("stack trace printing feature has been disabled");

    // create our own pid plugin
    pub struct PidPlugin { pub pid: u32 }

    impl logkit::Plugin for PidPlugin {
        fn post(&self, record: &mut logkit::Record) -> bool {
            record.append("pid", self.pid);
            true
        }
    }

    logkit::default_logger_mut().mount("pid", Box::new(PidPlugin {pid: std::process::id()}));

    info!("you will see this log with a process id");
}
```

## Documentation

The documentation is [available here](https://docs.rs/logkit).

## License

This software is released under the [MIT License](https://github.com/chensoft/logkit/blob/master/LICENSE).