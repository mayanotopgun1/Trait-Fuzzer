use std::hint;

struct U16(#[allow(dead_code)] u16);

impl Drop for U16 {
    fn drop(&mut self) {

        assert!(hint::black_box(self as *mut U16 as usize) % 2 == 0);
    }
}

trait DropTrait {}
impl DropTrait for U16 {}

struct HasDrop;

impl Drop for HasDrop {
    fn drop(&mut self) {}
}
impl DropTrait for HasDrop {}

struct Wrapper {
    _a: U16,
    b: HasDrop,
}

#[repr(packed)]
struct Misalign(#[allow(dead_code)] u8, Wrapper);

impl Misalign {
    fn new(inner: Wrapper) -> Self {
        Misalign(0, inner)
    }
}

fn main() {
    let m = Misalign::new(
        Wrapper {
            _a: U16(10),
            b: HasDrop,
        },
    );

    let m: ([u16; 0], Misalign) = ([], m);

    let _x = m.1.1.b;
}