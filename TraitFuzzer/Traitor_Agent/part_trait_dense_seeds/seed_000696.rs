#![allow(dead_code)]
#![allow(improper_ctypes)]

struct TwoDoubles {
    r: f64,
    i: f64
}

trait Identity {
    unsafe fn identity(self) -> Self;
}

impl Identity for TwoDoubles {
    unsafe fn identity(self) -> Self {
        rust_dbg_extern_identity_TwoDoubles(self)
    }
}

extern "C" {
    fn rust_dbg_extern_identity_TwoDoubles(arg1: TwoDoubles) -> TwoDoubles;
}

pub fn main() {}