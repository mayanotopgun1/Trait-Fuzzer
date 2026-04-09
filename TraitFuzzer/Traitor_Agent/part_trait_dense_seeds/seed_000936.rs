#![feature(dyn_trait)]

trait Gen<T> {
    type Out;
    fn gen(&self) -> Self::Out where Self: 'static;
}

trait GenExt<T>: Gen<T> where T: Default + Clone {
    fn gen_default(&self) -> T {
        T::default()
    }
}

impl<U, T> GenExt<T> for U where U: Gen<T>, T: Default + Clone {}

struct A;

impl Gen<[(); 0]> for A {
    type Out = [(); 0];
    fn gen(&self) -> Self::Out {
        []
    }
}

trait ArrayGen: Gen<[(); 0]> + GenExt<[(); 0]> where <Self as Gen<[(); 0]>>::Out: Default {}

impl<T> ArrayGen for T where T: Gen<[(); 0]> + GenExt<[(); 0]>, <T as Gen<[(); 0]>>::Out: Default {}

fn array() -> Box<dyn ArrayGen<Out = [(); 0]>> {
    Box::new(A)
}

fn main() {
    let a = A;
    let _x: [(); 0] = <A as Gen<[(); 0]>>::gen(&a);
}