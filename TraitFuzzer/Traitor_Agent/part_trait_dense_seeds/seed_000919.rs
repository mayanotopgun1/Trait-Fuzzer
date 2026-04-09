#![crate_type = "lib"]
#![crate_name = "issue48984aux"]

pub trait Foo {
    type Item<'a>;
}

pub trait Bar: for<'a> Foo<Item<'a>=[u8;1]> {
    fn bar_method(&self) -> &[u8; 1] {
        &[0u8]
    }
}

trait BarView: Bar {
    fn view_bar(&self) -> Option<&[u8; 1]> {
        Some(self.bar_method())
    }
}

impl<T> BarView for T where T: Bar {}

impl<T> Bar for T where T: for<'a> Foo<Item<'a>=[u8;1]> {}