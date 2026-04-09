#![allow(non_upper_case_globals)]
#![allow(unpredictable_function_pointer_comparisons)]

extern "C" fn foopy() {}

static f: extern "C" fn() = foopy;
static s: S = S { f: foopy };

struct S {
    f: extern "C" fn()
}

trait FunctionPointerAccess {
    fn get_function(&self) -> extern "C" fn();
}

impl FunctionPointerAccess for S {
    fn get_function(&self) -> extern "C" fn() {
        self.f
    }
}

pub fn main() {
    assert!(foopy as extern "C" fn() == f);
    assert!(f == s.get_function());
}