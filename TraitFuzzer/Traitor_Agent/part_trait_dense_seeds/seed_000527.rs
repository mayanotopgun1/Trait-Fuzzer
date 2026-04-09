#![crate_type = "rlib"]
#![crate_name = "foo"]

trait FooTrait { fn foo() -> i32; }
impl FooTrait for () { fn foo() -> i32 { 1 } }

pub fn foo() -> i32 {
    <() as FooTrait>::foo()
}