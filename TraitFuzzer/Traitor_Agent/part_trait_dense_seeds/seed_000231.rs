#![feature(const_trait_impl)]
#![feature(min_specialization)]

enum Foo {
    A = 5,
    B = 42,
}
enum Bar {
    C = 42,
    D = 99,
}

const trait UnionTrait {
    unsafe fn foo(&self) -> &'static Foo;
    unsafe fn bar(&self) -> &'static Bar;
}

#[repr(C)]
union Union {
    foo: &'static Foo,
    bar: &'static Bar,
    u8: &'static u8,
}

impl const UnionTrait for Union where Self: Sized {
    unsafe fn foo(&self) -> &'static Foo {
        self.foo
    }

    unsafe fn bar(&self) -> &'static Bar {
        self.bar
    }
}

const trait UnionTraitExt: UnionTrait {
    unsafe fn as_pair(&self) -> (&'static Foo, &'static Bar);
}

impl const UnionTraitExt for Union where Self: Sized {
    unsafe fn as_pair(&self) -> (&'static Foo, &'static Bar) {
        (self.foo(), self.bar())
    }
}

static BAR: u8 = 5;

static FOO: (&Foo, &Bar) = unsafe {
    let union_instance = Union { u8: &BAR };
    union_instance.as_pair()
};

static FOO2: (&Foo, &Bar) = unsafe { (std::mem::transmute(&BAR), std::mem::transmute(&BAR)) };

fn main() {}