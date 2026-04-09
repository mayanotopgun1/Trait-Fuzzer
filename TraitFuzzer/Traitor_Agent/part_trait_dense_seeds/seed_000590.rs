#![allow(dead_code)]

pub enum Foo {
    Bar,
    Baz(&'static str),
}

impl Foo {
    pub fn is_bar(&self) -> bool {
        matches!(self, Foo::Bar)
    }

    pub fn get_str(&self) -> Option<&'static str> {
        match self {
            Foo::Baz(s) => Some(s),
            _ => None,
        }
    }
}

pub trait FooExt {
    fn is_bar(&self) -> bool;
    fn get_str(&self) -> Option<&'static str>;
}

impl FooExt for Foo {
    fn is_bar(&self) -> bool {
        self.is_bar()
    }

    fn get_str(&self) -> Option<&'static str> {
        self.get_str()
    }
}

pub static TEST: Test = Test {
    foo: Foo::Bar,
    c: 'a'
};

pub struct Test {
    foo: Foo,
    c: char,
}

impl Test {
    pub fn get_foo(&self) -> &Foo {
        &self.foo
    }

    pub fn get_char(&self) -> char {
        self.c
    }
}

pub trait TestExt {
    fn get_foo(&self) -> &Foo;
    fn get_char(&self) -> char;
}

impl TestExt for Test {
    fn get_foo(&self) -> &Foo {
        self.get_foo()
    }

    fn get_char(&self) -> char {
        self.get_char()
    }
}

fn main() {}