#![feature(generic_associated_types)]

struct A;

trait AFn {
    type Call<'a> where Self: 'a;
    fn call(self, a: A) -> Self::Call<'static>;
}

impl AFn for fn(A) {
    type Call<'a> = ();
    fn call(self, a: A) -> Self::Call<'static> { self(a); }
}

fn fna(_a: A) {}

trait AFnExt: AFn where Self: Sized + 'static + Clone {
    fn call_twice(&self, a: A, b: A) where Self::Call<'static>: Copy {
        let _ = self.clone().call(a);
        let _ = self.clone().call(b);
    }
}

impl<T: AFn + 'static + Clone> AFnExt for T {}

#[allow(unpredictable_function_pointer_comparisons)]
fn main() {
    let fa: fn(A) = fna;
    fa.call_twice(A, A);
}