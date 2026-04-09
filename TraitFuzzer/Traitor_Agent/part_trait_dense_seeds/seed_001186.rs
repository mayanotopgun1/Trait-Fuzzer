#![crate_type = "lib"]
#![feature(naked_functions_rustic_abi)]

use std::arch::naked_asm;

trait NakedFunction {
    unsafe fn call(&self, p: char) -> u128;
}

impl NakedFunction for () {
    #[unsafe(naked)]
    unsafe fn call(&self, _p: char) -> u128 {
        naked_asm!("")
    }
}

fn main() {
    let func = ();
    unsafe {
        let _ = func.call('a');
    }
}