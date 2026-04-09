#![feature(specialization)]
#![allow(dead_code)]

trait CharAccess {
    fn get_char(&self) -> &'static char;
}

default impl<T> CharAccess for T {
    fn get_char(&self) -> &'static char {
        &'X'
    }
}

impl CharAccess for () {
    fn get_char(&self) -> &'static char {
        &'A'
    }
}

pub fn main() {
    let _ = ().get_char();
}