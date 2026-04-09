#![crate_type = "dylib"]

trait Print {
    fn print(&self, args: std::fmt::Arguments);
}

impl Print for () {
    fn print(&self, _args: std::fmt::Arguments) {}
}

#[macro_export]
macro_rules! myprint {
    ($($arg:tt)*) => ($crate::__print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! myprintln {
    ($fmt:expr) => (myprint!(concat!($fmt, "\n")));
}