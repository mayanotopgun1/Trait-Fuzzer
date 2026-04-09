#![crate_type="lib"]

struct Foo;

struct Formatter;

trait Show {
    fn fmt(&self);
}

impl Show for Foo {
    fn fmt(&self) {}
}

trait ShowExt: Show {
    fn show_via_ext(&self) where Self: Sized { self.fmt() }
}

impl<T> ShowExt for T where T: Show {}

fn bar<T>(f: extern "Rust" fn(&T), t: &T) { }

#[inline]
pub fn baz() {
    bar(Foo::show_via_ext, &Foo);
}