#![crate_type = "lib"]
#![allow(unconditional_panic)]

trait Accessor<'a> {
    type Out: 'a;
    fn access(&'a self, index: usize) -> &'a Self::Out;
}

trait AccessExt<'a>: Accessor<'a> where Self::Out: Copy {
    fn double_access(&'a self, index: usize) -> (Self::Out, Self::Out) {
        let value = self.access(index);
        (*value, *value)
    }
}

impl<'a, T> AccessExt<'a> for T where T: Accessor<'a>, T::Out: Copy {}

struct S(u8);

impl<'a> Accessor<'a> for S {
    type Out = u8;
    fn access(&'a self, _index: usize) -> &'a Self::Out {
        &self.0
    }
}

pub fn ice() {
    let s = S(0);
    let _ = s.double_access(0);
}