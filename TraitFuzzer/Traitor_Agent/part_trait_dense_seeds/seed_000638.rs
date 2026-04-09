#![feature(impl_trait_in_assoc_type)]

fn main() {}

#[cfg(false)]
trait A {
    const _: () = ();
}

#[cfg(false)]
impl A for () {
    const _: () = ();
}

#[cfg(false)]
trait DynA: A {}
#[cfg(false)]
impl<T: A> DynA for T {}

#[cfg(false)]
const _: () = {
    impl dyn DynA {
        const _: () = ();
    }
};