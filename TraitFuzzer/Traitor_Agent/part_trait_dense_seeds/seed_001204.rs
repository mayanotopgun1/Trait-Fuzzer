use std::fmt::Debug;

trait Foo {
    fn foo(&self) -> impl Debug;
}

impl Foo for () {
    fn foo(&self) -> impl Debug {
        "Hello, world"
    }
}

impl<T: Default + Debug> Foo for std::marker::PhantomData<T> {
    fn foo(&self) -> impl Debug {
        T::default()
    }
}

trait Bar {
    fn bar<T>(&self) -> impl Debug;
}

impl Bar for () {
    fn bar<T>(&self) -> impl Debug {
        format!("Hello with generic {}", std::any::type_name::<T>())
    }
}

trait Baz {
    fn baz(&self) -> impl Debug + '_;
}

impl Baz for String {
    fn baz(&self) -> impl Debug + '_ {
        (self,)
    }
}

trait FooExt: Foo {
    fn foo_ext(&self) -> impl Debug {
        format!("Extended foo: {:?}", self.foo())
    }
}

impl<T> FooExt for T where T: Foo {}

trait BarExt: Bar {
    fn bar_ext<T>(&self) -> impl Debug {
        format!("Extended bar: {:?}", self.bar::<T>())
    }
}

impl<T> BarExt for T where T: Bar {}

trait BazExt: Baz {
    fn baz_ext(&self) -> impl Debug + '_ {
        format!("Extended baz: {:?}", self.baz())
    }
}

impl<T> BazExt for T where T: Baz {}

fn main() {
    let unit = ();
    println!("{:?}", unit.foo());
    println!("{:?}", unit.bar::<u64>());
    println!("{:?}", "hi".to_string().baz());

    println!("{:?}", unit.foo_ext());
    println!("{:?}", unit.bar_ext::<u64>());
    println!("{:?}", "hi".to_string().baz_ext());
}