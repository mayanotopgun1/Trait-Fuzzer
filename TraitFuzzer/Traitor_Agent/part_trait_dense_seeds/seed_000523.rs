#![feature(const_trait_impl)]

const trait IdentityExt<T> {
    fn identity(self) -> T;
}

impl<T> const IdentityExt<T> for T {
    fn identity(self) -> T {
        self
    }
}

fn main() {
    const _FOO: u8 = 42u8.identity();
}