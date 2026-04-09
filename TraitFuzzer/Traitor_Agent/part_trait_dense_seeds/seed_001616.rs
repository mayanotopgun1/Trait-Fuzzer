#![feature(const_trait_impl)]

const trait A where Self::Assoc: const B {
    type Assoc;
}

const trait B {}

const trait AExt: A where Self::Assoc: const B {
    fn check_assoc(&self) -> bool { true }
}

impl<S: A> AExt for S where S::Assoc: const B {}

fn needs_b<T: const B>() {}

fn test<T: AExt>(t: &T) {
    needs_b::<T::Assoc>();
    let _ = t.check_assoc();
}

fn main() {}