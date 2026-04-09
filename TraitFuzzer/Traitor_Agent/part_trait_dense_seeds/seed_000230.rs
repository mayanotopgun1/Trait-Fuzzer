trait Foo {
    const X: i32;
    fn get_x(&self) -> i32 {
        Self::X
    }
}

struct Abc;
impl Foo for Abc {
    const X: i32 = 11;
}

struct Def;
impl Foo for Def {
    const X: i32 = 97;
}

struct Proxy<T>(#[allow(dead_code)] T);

impl<T: Foo> Foo for Proxy<T> {
    const X: i32 = T::X;
}

trait Bar: Foo {
    const Y: i32 = Self::X;
}

fn sub<A: Foo, B: Foo>() -> i32 {
    A::X - B::X
}

trait FooExt: Foo {
    fn get_x_static() -> i32 {
        Self::X
    }
}

impl<T: Foo> FooExt for T {}

fn main() {
    assert_eq!(11, Abc::get_x_static());
    assert_eq!(97, Def::get_x_static());
    assert_eq!(11, Abc.get_x());
    assert_eq!(97, Def.get_x());
    assert_eq!(-86, sub::<Abc, Def>());
    assert_eq!(86, sub::<Def, Abc>());
    assert_eq!(-86, sub::<Proxy<Abc>, Def>());
    assert_eq!(-86, sub::<Abc, Proxy<Def>>());
    assert_eq!(86, sub::<Proxy<Def>, Proxy<Abc>>());
}