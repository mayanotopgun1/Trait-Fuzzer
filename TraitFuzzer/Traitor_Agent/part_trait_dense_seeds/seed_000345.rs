#![feature(naked_functions_rustic_abi, type_alias_impl_trait, specialization)]
#![crate_type = "lib"]

use std::arch::naked_asm;

trait NakedFunction {
    type CallType;
    unsafe fn call(&self, p: char) -> Self::CallType;
}

type OpaqueCallType = u128;

default impl<T> NakedFunction for T {
    type CallType = OpaqueCallType;

    #[unsafe(naked)]
    unsafe fn call(&self, _p: char) -> Self::CallType {
        naked_asm!("", options())
    }
}

impl NakedFunction for () {
    type CallType = OpaqueCallType;

    #[unsafe(naked)]
    unsafe fn call(&self, _p: char) -> Self::CallType {
        naked_asm!("", options())
    }
}

trait CallTrait {
    unsafe fn do_call(&self, p: char) -> OpaqueCallType;
}

default impl<T: NakedFunction<CallType = u128>> CallTrait for T {
    unsafe fn do_call(&self, p: char) -> OpaqueCallType {
        self.call(p)
    }
}

impl CallTrait for () {
    unsafe fn do_call(&self, p: char) -> OpaqueCallType {
        self.call(p)
    }
}

pub extern "C" fn naked(p: char) -> u128 {
    unsafe { ().do_call(p) }
}