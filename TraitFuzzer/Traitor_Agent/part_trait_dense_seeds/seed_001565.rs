#![allow(non_camel_case_types)]
#![feature(impl_trait_in_assoc_type)]

trait foo {
    fn foo(&self) -> i32;
}

impl foo for Vec<u32> {
    fn foo(&self) -> i32 { 1 }
}

impl foo for Vec<i32> {
    fn foo(&self) -> i32 { 2 }
}

trait foo_ext: foo {
    fn extended_foo(&self) -> (i32, i32) {
        let v = self.foo();
        (v, v)
    }
}

impl<T: foo> foo_ext for T {}

fn call_foo_uint() -> impl foo_ext {
    let mut x: Vec<u32> = Vec::new();
    let _y = x.extended_foo().0;
    x.push(0u32);
    x
}

fn call_foo_int() -> impl foo_ext {
    let mut x: Vec<i32> = Vec::new();
    let _y = x.extended_foo().0;
    x.push(0i32);
    x
}

fn main() {
    assert_eq!(call_foo_uint().extended_foo().0, 1);
    assert_eq!(call_foo_int().extended_foo().0, 2);
}