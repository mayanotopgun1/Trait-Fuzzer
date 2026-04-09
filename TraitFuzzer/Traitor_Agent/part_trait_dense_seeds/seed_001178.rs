#![feature(return_position_impl_trait_in_trait)]
#![allow(static_mut_refs)]

static mut DROPPED: [bool; 2] = [false, false];

struct A(usize);
struct Foo { _a: A, _b: isize }

trait DropTrait {
    fn custom_drop(&mut self) -> impl core::fmt::Debug;
}

impl Drop for A {
    fn drop(&mut self) {
        let A(i) = *self;
        unsafe { DROPPED[i] = true; }
    }
}

impl DropTrait for Foo {
    fn custom_drop(&mut self) -> impl core::fmt::Debug {
        std::mem::drop(&mut self._a);
        true
    }
}

fn main() {
    {
        let mut foo = Foo {
            _a: A(0),
            ..Foo { _a: A(1), _b: 2 }
        };
        let _ = foo.custom_drop();
    }
    unsafe {
        assert!(DROPPED[0]);
        assert!(DROPPED[1]);
    }
}