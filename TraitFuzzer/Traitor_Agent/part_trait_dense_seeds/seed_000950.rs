#![feature(try_as_dyn)]

use std::fmt::{Error, Write};

trait DynWrite {
    fn dyn_write_str(&mut self, s: &str) -> Result<(), Error>;
}

impl<T> DynWrite for T
where
    T: 'static + Write,
{
    fn dyn_write_str(&mut self, s: &str) -> Result<(), Error> {
        self.write_str(s)
    }
}

fn try_as_dyn_mut_write<T: 'static>(t: &mut T, s: &str) -> Result<(), Error>
where
    T: DynWrite,
{
    t.dyn_write_str(s)
}

fn main() {
    let mut buf = "Hello".to_string();

    try_as_dyn_mut_write(&mut buf, " world!").unwrap();
    assert_eq!(buf, "Hello world!");
}