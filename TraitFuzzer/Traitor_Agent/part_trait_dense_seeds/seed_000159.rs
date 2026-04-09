#![feature(const_trait_impl)]

struct S(pub &'static u32, pub u32);

const trait RefAccess {
    fn get_ref(&self) -> &u32;
}

impl const RefAccess for S {
    fn get_ref(&self) -> &u32 {
        &self.1
    }
}

const fn g(ss: &S) -> &u32 { ss.get_ref() }

static T: S = S(g(&T), 0);

fn main () { }