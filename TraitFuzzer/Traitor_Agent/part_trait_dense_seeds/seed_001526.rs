#![allow(dead_code)]
#[derive(Copy, Clone)]
struct Functions {
    a: fn(u32) -> u32,
    b: extern "C" fn(u32) -> u32,
    c: unsafe fn(u32) -> u32,
    d: unsafe extern "C" fn(u32) -> u32,
}

trait FunctionCaller {
    fn call_a(&self, x: u32) -> u32;
    fn call_b(&self, x: u32) -> u32;
    unsafe fn call_c(&self, x: u32) -> u32;
    unsafe fn call_d(&self, x: u32) -> u32;
}

impl FunctionCaller for Functions {
    fn call_a(&self, x: u32) -> u32 {
        (self.a)(x)
    }

    fn call_b(&self, x: u32) -> u32 {
        (self.b)(x)
    }

    unsafe fn call_c(&self, x: u32) -> u32 {
        (self.c)(x)
    }

    unsafe fn call_d(&self, x: u32) -> u32 {
        (self.d)(x)
    }
}

pub fn main() {}