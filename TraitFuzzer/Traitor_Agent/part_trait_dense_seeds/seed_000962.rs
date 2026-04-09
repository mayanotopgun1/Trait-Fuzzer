#[repr(transparent)]
#[derive(Copy, Clone)]
struct MyPhantom(core::marker::PhantomData<u8>);

trait PhantomTrait {
    fn phantom_data(&self) -> core::marker::PhantomData<u8>;
}

impl PhantomTrait for MyPhantom {
    fn phantom_data(&self) -> core::marker::PhantomData<u8> {
        self.0
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Bar {
    pub x: i32,
    _marker: MyPhantom,
}

impl PhantomTrait for Bar {
    fn phantom_data(&self) -> core::marker::PhantomData<u8> {
        self._marker.phantom_data()
    }
}

extern "C" {
    pub fn foo(bar: *mut Bar);
}

fn main() {}