trait Super {}
trait Sub<T>: Super {}

struct W<T>(T);
trait Bound<T> {}
impl<T: Sub<U>, U> Bound<W<U>> for T {}

trait Overlap<T> {}
impl<T, U: Bound<W<T>>> Overlap<T> for U {}
impl<T> Overlap<T> for () {}

trait Usage {
    fn check_bound(&self);
}

impl<T, U> Usage for (T, U)
where
    T: Overlap<U>,
{
    fn check_bound(&self) {}
}

fn main() {}