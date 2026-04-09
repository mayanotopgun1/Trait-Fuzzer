#![feature(generic_associated_types)]

use std::cell::Cell;

const NONE_CELL_STRING: Option<Cell<String>> = None;

struct Foo<T>(#[allow(dead_code)] T);
impl<T> Foo<T> {
    const FOO: Option<Box<T>> = None;
}

trait CellOptionExt<'a> {
    type Item;
    fn is_none(&self) -> bool;
}

impl<'a, T: 'a> CellOptionExt<'a> for Option<Cell<T>> {
    type Item = &'a Cell<T>;
    fn is_none(&self) -> bool {
        self.is_none()
    }
}

trait BoxOptionExt<'a> {
    type Item;
    fn is_none(&self) -> bool;
}

impl<'a, T: 'a> BoxOptionExt<'a> for Option<Box<T>> {
    type Item = &'a Box<T>;
    fn is_none(&self) -> bool {
        self.is_none()
    }
}

trait OptionExt<'a> {
    type Item;
    fn is_none_like(&self) -> bool where Self: CellOptionExt<'a> + BoxOptionExt<'a>;
}

impl<'a, T: 'a> OptionExt<'a> for Option<Cell<T>> {
    type Item = &'a Cell<T>;
    fn is_none_like(&self) -> bool {
        self.is_none()
    }
}

impl<'a, T: 'a> OptionExt<'a> for Option<Box<T>> {
    type Item = &'a Box<T>;
    fn is_none_like(&self) -> bool {
        self.is_none()
    }
}

fn main() {
    let _: &'static u32 = &42;
    let _: &'static Option<u32> = &None;

    let _: &'static Option<Cell<String>> = &NONE_CELL_STRING;
    let _: &'static Option<Box<()>> = &Foo::FOO;

    assert!(NONE_CELL_STRING.is_none());
    assert!(Foo::<Box<()>>::FOO.is_none());
}