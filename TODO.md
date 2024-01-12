* read log level from env
* specify stdout or stderr
* limit level in compile time using features
* default static logger and default macros
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
* multiple output targets
* add benchmarks