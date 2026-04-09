#![allow(dead_code)]

trait Baz {
}

trait Bar<T> {
}

trait BarExt<T>: Bar<T> {
    fn bar_ref(&self) -> &dyn Bar<T>;
}

impl<T, U> BarExt<U> for T where T: Bar<U> {
    fn bar_ref(&self) -> &dyn Bar<U> {
        self
    }
}

trait BazExt: Baz {
    fn baz_ref(&self) -> &dyn Baz;
}

impl<T> BazExt for T where T: Baz {
    fn baz_ref(&self) -> &dyn Baz {
        self
    }
}

fn make_bar<T:Bar<u32>>(t: &T) -> &dyn Bar<u32> {
    t.bar_ref()
}

fn make_baz<T:Baz>(t: &T) -> &dyn Baz {
    t.baz_ref()
}

fn main() {
}