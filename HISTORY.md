## [Unreleased]

### Todo

- more friendly to get mut logger
- limit level in compile time using features
- async write support and thread local
- log rotate by filesize, lineno, daily, hourly...
- color support in console output
- highlight keywords in console output
- sampling by level

## [0.1.1] - 2024-01-24

### Added

- benchmarks

### Changed

- `record` macro accept custom logger

## [0.1.0] - 2024-01-24

### Added

- JSON encoding output
- flexible plugin system
- multiple output targets
- extremely fast encoding speed
- predefined default logger
- customizable logger object
- logging with an optional stack trace
- the output order of the fields is fixed
- the plugin can cancel the output of a log entry midway