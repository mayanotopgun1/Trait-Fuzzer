#![allow(dead_code)]

trait CharAccess {
    fn get_char(&self) -> &'static char;
}

impl CharAccess for () {
    fn get_char(&self) -> &'static char {
        &'A'
    }
}

pub fn main() {
    let _ = ().get_char();
}