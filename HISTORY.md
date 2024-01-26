## [Unreleased]

### Todo

- predefined fields do not need invoked every time
- limit level in compile time using features
- async write support and thread local
- log rotate by filesize, lineno, daily, hourly...
- color support in console output
- highlight keywords in console output
- sampling by level

## [0.3.0] - 2024-01-xx

### Added

- a nop constructor for Logger
- a DiscardTarget for ignoring output

### Changed

- `new` in Logger requires a default target

### Removed

- `from_env` in Logger
- `from_def` in Logger

## [0.2.0] - 2024-01-26

### Added

- benchmark
- const constructor for Logger

### Changed

- unmount a plugin using a callback
- unroute a target using a callback
- macros support trailing commas
- change default output target to stderr
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