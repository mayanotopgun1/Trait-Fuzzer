#![allow(dead_code)]

struct Foo {
    #[cfg(false)]
    bar: baz,
    foo: isize,
}

impl Foo {
    fn get_foo(&self) -> isize {
        self.foo
    }
}

trait FooExt {
    fn double_foo(&self) -> isize;
}

impl FooExt for Foo {
    fn double_foo(&self) -> isize {
        self.get_foo() + self.get_foo()
    }
}

struct Foo2 {
    #[cfg(true)]
    foo: isize,
}

impl Foo2 {
    fn get_foo(&self) -> isize {
        self.foo
    }
}

trait Foo2Ext {
    fn double_foo(&self) -> isize;
}

impl Foo2Ext for Foo2 {
    fn double_foo(&self) -> isize {
        self.get_foo() + self.get_foo()
    }
}

enum Bar1 {
    Bar1_1,
    #[cfg(false)]
    Bar1_2(NotAType),
}

trait Bar1Ext {}

impl Bar1Ext for Bar1 {}

enum Bar2 {
    #[cfg(false)]
    Bar2_1(NotAType),
}

trait Bar2Ext {}

impl Bar2Ext for Bar2 {}

enum Bar3 {
    Bar3_1 {
        #[cfg(false)]
        foo: isize,
        bar: isize,
    }
}

impl Bar3 {
    fn get_bar(&self) -> isize {
        match self {
            Bar3::Bar3_1 { bar, .. } => *bar,
        }
    }
}

trait Bar3Ext {
    fn double_bar(&self) -> isize;
}

impl Bar3Ext for Bar3 {
    fn double_bar(&self) -> isize {
        self.get_bar() + self.get_bar()
    }
}

pub fn main() {
    let _f = Foo { foo: 3 };
    let _f = Foo2 { foo: 3 };

    match Bar1::Bar1_1 {
        Bar1::Bar1_1 => {}
    }

    let _f = Bar3::Bar3_1 { bar: 3 };
}