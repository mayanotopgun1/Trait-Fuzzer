#[repr(packed)]
struct Foo1 {
    bar: u8,
    baz: usize
}

trait UnpackFoo1 {
    fn unpack(&self) -> (u8, usize);
}

impl UnpackFoo1 for Foo1 {
    fn unpack(&self) -> (u8, usize) {
        (self.bar, self.baz)
    }
}

#[repr(packed(2))]
struct Foo2 {
    bar: u8,
    baz: usize
}

trait UnpackFoo2 {
    fn unpack(&self) -> (u8, usize);
}

impl UnpackFoo2 for Foo2 {
    fn unpack(&self) -> (u8, usize) {
        (self.bar, self.baz)
    }
}

#[repr(C, packed(4))]
struct Foo4C {
    bar: u8,
    baz: usize
}

trait UnpackFoo4C {
    fn unpack(&self) -> (u8, usize);
}

impl UnpackFoo4C for Foo4C {
    fn unpack(&self) -> (u8, usize) {
        (self.bar, self.baz)
    }
}

pub fn main() {
    let foo1 = Foo1 { bar: 1, baz: 2 };
    match foo1.unpack() {
        (bar, baz) => {
            assert_eq!(bar, 1);
            assert_eq!(baz, 2);
        }
    }

    let foo2 = Foo2 { bar: 1, baz: 2 };
    match foo2.unpack() {
        (bar, baz) => {
            assert_eq!(bar, 1);
            assert_eq!(baz, 2);
        }
    }

    let foo4 = Foo4C { bar: 1, baz: 2 };
    match foo4.unpack() {
        (bar, baz) => {
            assert_eq!(bar, 1);
            assert_eq!(baz, 2);
        }
    }
}