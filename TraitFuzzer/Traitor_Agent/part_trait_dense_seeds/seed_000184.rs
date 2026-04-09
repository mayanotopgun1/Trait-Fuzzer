use std::ffi::{c_double, c_int};
use std::mem;

trait TransmuteCopy {
    fn to_c_int(&mut self) -> &mut c_int;
}

impl TransmuteCopy for isize {
    fn to_c_int(&mut self) -> &mut c_int {
        unsafe { mem::transmute_copy(&self) }
    }
}

fn lgamma(n: c_double, value: &mut isize) -> c_double {
    unsafe {
        m::lgamma(n, value.to_c_int())
    }
}

mod m {
    use std::ffi::{c_double, c_int};
    extern "C" {
        #[cfg(all(unix, not(target_os = "vxworks")))]
        #[link_name = "lgamma_r"]
        pub fn lgamma(n: c_double, sign: &mut c_int) -> c_double;
        #[cfg(windows)]
        #[link_name = "lgamma"]
        pub fn lgamma(n: c_double, sign: &mut c_int) -> c_double;
        #[cfg(target_os = "vxworks")]
        #[link_name = "lgamma"]
        pub fn lgamma(n: c_double, sign: &mut c_int) -> c_double;
    }
}

pub fn main() {
    let mut y: isize = 5;
    let x: &mut isize = &mut y;
    assert_eq!(lgamma(1.0 as c_double, x), 0.0 as c_double);
}