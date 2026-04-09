#![feature(type_alias_impl_trait)]
#![allow(dead_code)]

trait Access<'a> {
    type Ref;
    fn value(&self) -> Self::Ref;
}

trait AccessView<'a>: Access<'a> {
    fn get_value_ref(&self) -> Self::Ref {
        self.value()
    }
}

impl<'a, T: Access<'a>> AccessView<'a> for T {}

struct S<'a> {
    v: &'a isize
}

impl<'a> Access<'a> for S<'a> {
    type Ref = &'a isize;
    fn value(&self) -> Self::Ref {
        self.v
    }
}

fn f<'lt, T: AccessView<'lt>>(_s: &'lt T) {}

pub fn main() {
    let s = S { v: &42 };
    f(&s);
}