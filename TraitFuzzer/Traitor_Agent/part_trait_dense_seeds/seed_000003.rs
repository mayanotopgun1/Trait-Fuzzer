static FOO: Foo = Foo {
    field: &42 as *const i32,
};

struct Foo {
    field: *const i32,
}

trait FieldAccess {
    unsafe fn get_field(&self) -> i32;
}

unsafe impl Sync for Foo {}

impl FieldAccess for Foo {
    unsafe fn get_field(&self) -> i32 {
        *self.field
    }
}

fn main() {
    assert_eq!(unsafe { FOO.get_field() }, 42);
}