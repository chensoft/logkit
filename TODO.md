* specify stdout or stderr
* limit level in compile time using features
* user can create custom logger
* switch between plain log and structure log
* log can have prefix
* custom prefix format and named params
* builtin named params like time, pid, file, lineno...
* guaranteed output fields order
* allow providing functions to customize named params
* async write support and thread local
* log rotate by filesize, filenum, daily, hourly...
* record stacktrace if enabled for some levels
* hook output function allow to continue or discard
* hook output by level
* hook allow to modify output text
* multiple output targets
* add benchmarks
* time format like 2022-08-31T21:00:29.123+01:00
* allow to set timezone, default use local tz
* multiple output method, default json
* console output like 2022-09-01T10:03:06.123+01:00 D main.rs:15 > debug message pid=12345 foo=bar
* color support in console output
* highlight keywords in console output
* sampling by level
* fatal level handler, default panic
* disable levels using features at compile time
* allow log! return string
* allow log! close output
* no std support?