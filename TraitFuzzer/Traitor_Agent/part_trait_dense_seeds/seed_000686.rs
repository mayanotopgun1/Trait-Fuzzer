static FOO: Foo = Foo {
    field: &42 as *const i32,
};

struct Foo {
    field: *const i32,
}

trait FieldAccess {
    unsafe fn get_field(&self) -> i32;
}

impl FieldAccess for Foo {
    unsafe fn get_field(&self) -> i32 {
        *self.field
    }
}

unsafe impl Sync for Foo {}

trait FieldExt: FieldAccess {
    unsafe fn double_get_field(&self) -> i32 {
        self.get_field() + self.get_field()
    }
}

impl<T: FieldAccess> FieldExt for T {}

fn main() {
    unsafe {
        assert_eq!(FOO.double_get_field(), 84);
    }
}