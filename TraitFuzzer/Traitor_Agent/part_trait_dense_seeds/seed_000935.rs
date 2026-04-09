trait Gen<T> {
    type Out;
    fn gen(x: Self) -> Self::Out where Self: 'static;
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
    fn gen(x: Self) -> Self::Out {
        []
    }
}

trait ArrayGen: Gen<[(); 0]> + GenExt<[(); 0]> {}

impl<T> ArrayGen for T where T: Gen<[(); 0]> + GenExt<[(); 0]> {}

fn array() -> impl ArrayGen {
    A
}

fn main() {
    let _x: [(); 0] = <A as Gen<[(); 0]>>::gen(A);
}