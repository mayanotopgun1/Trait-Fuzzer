#![feature(type_alias_impl_trait, impl_trait_in_assoc_type, return_position_impl_trait_in_trait)]

use std::fmt;
use std::io::{self, Error, Write};
use std::panic::catch_unwind;

struct ErrorDisplay;

trait DisplayExt {
    fn fmt_ext(&self, _: &mut fmt::Formatter) -> fmt::Result;
}

impl DisplayExt for ErrorDisplay {
    fn fmt_ext(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Err(fmt::Error)
    }
}


impl fmt::Display for ErrorDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_ext(f)
    }
}

trait CustomWrite: Write {
    type Writer;
    fn custom_write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        let mut written = 0;
        while written < buf.len() {
            match self.write(&buf[written..]) {
                Ok(n) => written += n,
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

impl<T: Write> CustomWrite for T {
    type Writer = Self;
}

struct ErrorWriter;

const WRITER_ERROR: io::ErrorKind = io::ErrorKind::NotConnected;

impl Write for ErrorWriter {
    fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
        Err(Error::new(WRITER_ERROR, "not connected"))
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

trait FormatterExt {
    type Output;
    fn format_ext(&self, fmt::Arguments<'_>) -> Result<Self::Output, fmt::Error>;
}

impl FormatterExt for ErrorWriter {
    type Output = ();
    fn format_ext(&self, _args: fmt::Arguments<'_>) -> Result<(), fmt::Error> {
        Err(fmt::Error)
    }
}

fn main() {
    let mut ew = ErrorWriter;
    let res = ew.custom_write_all(b"abc");
    assert!(res.is_err(), "writer error did not propagate");

    let res = catch_unwind(|| write!(vec![], "{} {} {}", 1, ErrorDisplay, "bar"));
    let err = res.expect_err("formatter error did not lead to panic").downcast::<&str>().unwrap();
    assert!(
        err.contains("formatting trait implementation returned an error"),
        "unexpected panic: {}",
        err
    );

    let res = catch_unwind(|| write!(ErrorWriter, "{} abc", ErrorDisplay));
    let err = res.expect_err("formatter error did not lead to panic").downcast::<&str>().unwrap();
    assert!(
        err.contains("formatting trait implementation returned an error"),
        "unexpected panic: {}",
        err
    );
}