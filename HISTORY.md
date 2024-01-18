## [Unreleased]

### Todo

- limit level in compile time using features
- switch between plain log and structure log
- async write support and thread local
- log rotate by filesize, lineno, daily, hourly...
- console output like 2022-09-01T10:03:06.123+01:00 D main.rs:15 > debug message pid=12345 foo=bar
- color support in console output
- highlight keywords in console output
- sampling by level

### Added
### Fixed
### Changed
### Removed

## [0.1.0] - 2024-01-xx

### Added

- structured json encoded output
- flexible plugin system allows for predefined output content
- multiple targets can be combined for output to various locations
- extremely optimized for encoding to accelerate the logging speed
- plugins can terminate the logging process prematurely
- error logging with optional stacktrace
- easy-to-use default logging object
- predictable field output order