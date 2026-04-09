#![crate_type = "rlib"]
#![crate_name = "foo"]

trait FooTrait { fn foo(&self) -> i32; }
impl FooTrait for () { fn foo(&self) -> i32 { 1 } }

pub fn foo(x: Box<dyn FooTrait>) -> i32 {
    x.foo()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        assert_eq!(foo(Box::new(())), 1);
    }
}