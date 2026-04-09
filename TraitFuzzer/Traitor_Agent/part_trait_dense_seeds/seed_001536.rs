trait FooTrait<'a> {
    fn foo() -> *mut &'a ();
}

impl<'a> FooTrait<'a> for () {
    fn foo() -> *mut &'a () {
        let _: *mut &'a () = <Self as FooTrait<'a>>::foo();
        loop {}
    }
}

fn main() {}