trait BadMacroExt {
    fn bad_macro(&self, ex: i32) -> i32;
}

impl<T> BadMacroExt for T {
    fn bad_macro(&self, ex: i32) -> i32 {
        match 9 {
            _x => ex,
        }
    }
}

macro_rules! bad_macro { ($ex:expr) => (
    {let _this = (); _this.bad_macro($ex)}
)}

fn main() {
    match 8 {
        _x => assert_eq!(bad_macro!(_x), 8)
    }
}