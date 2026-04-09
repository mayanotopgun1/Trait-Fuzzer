#![deny(improper_ctypes)]

pub trait Foo {
    type Assoc: 'static;
}

impl Foo for () {
    type Assoc = u32;
}

pub trait BarExt<T: Foo> {
    fn get_value(&self) -> &'static <T as Foo>::Assoc;
}

impl<T: Foo> BarExt<T> for Bar<T> {
    fn get_value(&self) -> &'static <T as Foo>::Assoc {
        self.value
    }
}

pub trait BarClone<T: Foo> {
    fn clone_bar(&self) -> Self;
}

impl<T: Foo + Clone> BarClone<T> for Bar<T> {
    fn clone_bar(&self) -> Self {
        self.clone()
    }
}

extern "C" {
    pub fn lint_me(x: Bar<()>);
}

#[repr(transparent)]
pub struct Bar<T: Foo> {
    value: &'static <T as Foo>::Assoc,
}

impl<T: Foo> Clone for Bar<T> {
    fn clone(&self) -> Self {
        Bar { value: self.value }
    }
}

fn main() {}