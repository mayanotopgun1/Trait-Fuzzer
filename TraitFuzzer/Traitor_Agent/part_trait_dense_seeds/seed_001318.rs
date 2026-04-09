#![feature(generic_associated_types)]

struct Test;

trait TestTrait<'a> {
    type Out;
    
    #[allow(dead_code)]
    #[allow(unused_variables)]
    #[allow(improper_ctypes_definitions)]
    unsafe extern "C" fn test(&self, val: &'a str) -> Self::Out;
}

impl<'a> TestTrait<'a> for Test {
    type Out = &'a str;

    unsafe extern "C" fn test(&self, val: &'a str) -> Self::Out {
        val
    }
}

fn main() {}