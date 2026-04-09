#![allow(dead_code)]
#![allow(unused_variables)]

#![allow(static_mut_refs)]



#[derive(Copy, Clone)]
struct S;

union U {
    a: u8
}

union W {
    a: S,
}

union Y {
    a: S,
}

trait DropExt {
    fn custom_drop(&mut self);
}

impl Drop for U {
    fn drop(&mut self) {
        unsafe { CHECK += 1; }
    }
}

impl Drop for W {
    fn drop(&mut self) {
        unsafe { CHECK += 1; }
    }
}

impl DropExt for U {
    fn custom_drop(&mut self) {
        unsafe { CHECK += 1; }
    }
}

impl DropExt for W {
    fn custom_drop(&mut self) {
        unsafe { CHECK += 1; }
    }
}

static mut CHECK: u8 = 0;

fn main() {
    unsafe {
        assert_eq!(CHECK, 0);
        {
            let mut u = U { a: 1 };
            u.custom_drop();
        }
        assert_eq!(CHECK, 1);
        {
            let mut w = W { a: S };
            w.custom_drop();
        }
        assert_eq!(CHECK, 2);
        {
            let y = Y { a: S };
        }
        assert_eq!(CHECK, 2);
        {
            let u2 = U { a: 1 };
            std::mem::forget(u2);
        }
        assert_eq!(CHECK, 2);
    }
}