#![feature(const_trait_impl, generic_associated_types)]

const trait IdentityExt<T> {
    type Out;
    fn identity(self) -> Self::Out;
}

impl<T> const IdentityExt<T> for T {
    type Out = T;
    fn identity(self) -> Self::Out {
        self
    }
}

fn main() {
    const _FOO: u8 = 42u8.identity();
}