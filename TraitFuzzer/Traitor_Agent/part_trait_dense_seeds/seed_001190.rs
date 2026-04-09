#![allow(dead_code)]

struct MySlice<'a, T:'a>(&'a mut [T]);

impl<'a, T> MySlice<'a, T> {
    fn renew<'b: 'a>(self) -> &'b mut [T] where 'a: 'b {
        &mut self.0[..]
    }
}

trait SliceExt<'a, T:'a> {
    fn renew<'b: 'a>(self) -> &'b mut [T] where 'a: 'b;
}

impl<'a, T> SliceExt<'a, T> for MySlice<'a, T> {
    fn renew<'b: 'a>(self) -> &'b mut [T] where 'a: 'b {
        &mut self.0[..]
    }
}


fn main() { }