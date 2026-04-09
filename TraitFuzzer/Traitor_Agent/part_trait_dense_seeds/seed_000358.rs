#![feature(specialization)]

trait BadMacroExt {
    fn bad_macro(&self, ex: i32) -> i32;
}

default impl<T> BadMacroExt for T {
    fn bad_macro(&self, ex: i32) -> i32 {
        match 9 {
            _x => ex,
        }
    }
}

struct Specialized;

impl BadMacroExt for Specialized {
    fn bad_macro(&self, ex: i32) -> i32 {
        match 42 {
            _x => ex * 2,
        }
    }
}

macro_rules! bad_macro { ($ex:expr) => (
    {let _this = Specialized; _this.bad_macro($ex)}
)}

fn main() {
    match 8 {
        _x => assert_eq!(bad_macro!(_x), 16)
    }
}