#![feature(specialization)]

trait CharConversion {
    fn to_char(self) -> char;
}

default impl<T> CharConversion for T {
    default fn to_char(self) -> char {
        'd' // Default character if not specialized
    }
}

impl CharConversion for u8 {
    fn to_char(self) -> char {
        self as char
    }
}

pub fn main() {
    assert_eq!((0 + 0u8).to_char(), '\0');
}