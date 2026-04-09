#![allow(static_mut_refs)]

static mut DROPPED: [bool; 2] = [false, false];

struct A(usize);
struct Foo { _a: A, _b: isize }

trait DropTrait {
    fn custom_drop(&mut self);
}

impl Drop for A {
    fn drop(&mut self) {
        let A(i) = *self;
        unsafe { DROPPED[i] = true; }
    }
}

impl DropTrait for Foo {
    fn custom_drop(&mut self) {
        std::mem::drop(&mut self._a);
    }
}

fn main() {
    {
        let mut foo = Foo {
            _a: A(0),
            ..Foo { _a: A(1), _b: 2 }
        };
        foo.custom_drop();
    }
    unsafe {
        assert!(DROPPED[0]);
        assert!(DROPPED[1]);
    }
}