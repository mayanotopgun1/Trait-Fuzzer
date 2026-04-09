#![feature(type_alias_impl_trait)]

static FOO: Foo = Foo {
    field: &42 as *const i32,
};

struct Foo {
    field: *const i32,
}

trait FieldAccess {
    type Accessor;
    unsafe fn get_field(&self) -> Self::Accessor;
}

unsafe impl Sync for Foo {}

impl FieldAccess for Foo {
    type Accessor = i32;
    unsafe fn get_field(&self) -> Self::Accessor {
        *self.field
    }
}

fn main() {
    assert_eq!(unsafe { FOO.get_field() }, 42);
}