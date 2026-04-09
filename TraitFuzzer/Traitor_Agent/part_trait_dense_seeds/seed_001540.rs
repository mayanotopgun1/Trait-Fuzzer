#![feature(generic_associated_types)]

trait FooTrait<'a> {
    type Ref: 'a;
    fn foo(&self) -> *mut Self::Ref;
}

trait FooLoop<'a>: FooTrait<'a> {
    fn loop_forever(&self);
}

impl<'a, T> FooLoop<'a> for T where T: FooTrait<'a> {
    fn loop_forever(&self) {
        loop {}
    }
}

impl<'a> FooTrait<'a> for () {
    type Ref = &'a ();
    fn foo(&self) -> *mut Self::Ref {
        let _: *mut &'a () = 0 as *mut &'a ();
        self.loop_forever();

        0 as *mut &'a ()
    }
}

fn main() {}