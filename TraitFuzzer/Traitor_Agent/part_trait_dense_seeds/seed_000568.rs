use std::fmt::Debug;
use std::hint::black_box;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct Regular(f32, f64);

#[repr(C, packed)]
#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct Packed(f32, f64);

#[repr(C, align(64))]
#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct AlignedF32(f32);

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct Aligned(f64, AlignedF32);

trait Read<T> {
    extern "C" fn read(&self) -> T;
}

trait Write<T>: Read<T> {
    extern "C" fn write(self, dest: &mut T);
}

impl<T: Copy + Clone> Read<T> for T {
    extern "C" fn read(&self) -> T {
        *black_box(self)
    }
}

impl<T: Copy + Clone> Write<T> for T {
    extern "C" fn write(self, dest: &mut T) {
        *dest = black_box(self)
    }
}

#[track_caller]
fn check<T: Copy + PartialEq + Debug + Default + Read<T> + Write<T>>(x: T) {
    assert_eq!(x.read(), x);
    let mut out = T::default();
    x.write(&mut out);
    assert_eq!(out, x);
}

fn main() {
    check(Regular(1.0, 2.0));
    check(Packed(3.0, 4.0));
    check(Aligned(5.0, AlignedF32(6.0)));
}