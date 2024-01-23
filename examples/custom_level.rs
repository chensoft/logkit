fn main() {
    pub const LEVEL_CUSTOM : logkit::Level = 10; // use any number that does not conflict with builtins

    #[macro_export]
    macro_rules! custom {
        ($($arg:tt)*) => {{
            logkit::record!(LEVEL_CUSTOM, $($arg)*)
        }};
    }

    custom!("this is a custom log level");
}