### Todo

- predefined fields do not need invoked every time
- async write support and thread local
- log rotate by filesize, lineno, daily, hourly...
- color support in console output
- highlight keywords in console output
- sampling by level

## [Unreleased] - 2024-xx-xx

### Changed

- Remove the parking_lot dependency
- Relax the requirements for Plugin and Target

## [0.3.2] - 2024-03-07

### Added

- Retrieve logger's plugins and targets
- Automatically create the directory in FileTarget

### Changed

- Print an error message if writing to the file fails

## [0.3.1] - 2024-02-02

### Changed

- do not lock versions

## [0.3.0] - 2024-01-27

### Added

- a nop constructor for Logger
- a DiscardTarget for ignoring output
- features for macros to disable some levels in compile time

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