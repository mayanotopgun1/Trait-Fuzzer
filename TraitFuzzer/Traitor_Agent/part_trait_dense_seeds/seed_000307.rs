#![allow(unused)]
trait HigherRanked {
    type Output<'a>;
}
trait Id {
    type Refl: HigherRanked;
}

trait FooExt<T: Id>: Sized {
    fn foo(&self) -> for<'a> fn(<<T as Id>::Refl as HigherRanked>::Output<'a>);
}

impl<T: Id> FooExt<T> for () {
    fn foo(&self) -> for<'a> fn(<<T as Id>::Refl as HigherRanked>::Output<'a>) {
        todo!()
    }
}

fn bar<T: Id>() {

    let x = FooExt::<T>::foo(&());
}

fn main() {}