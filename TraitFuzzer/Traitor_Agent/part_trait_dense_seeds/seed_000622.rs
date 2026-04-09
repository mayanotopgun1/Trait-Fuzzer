#![allow(dead_code)]
#![feature(impl_trait_in_assoc_type)]

trait Structure<E>: Sized where E: Encoding {
    type RefTarget: ?Sized;
    type FfiPtr;
    unsafe fn borrow_from_ffi_ptr<'a>(ptr: Self::FfiPtr) -> Option<&'a Self::RefTarget>;
}

enum Slice {}

impl<E> Structure<E> for Slice where E: Encoding {
    type RefTarget = [E::Unit];
    type FfiPtr = (*const E::FfiUnit, usize);
    unsafe fn borrow_from_ffi_ptr<'a>(_ptr: Self::FfiPtr) -> Option<&'a Self::RefTarget> {
        panic!()
    }
}

trait Encoding {
    type Unit: Unit;
    type FfiUnit;
}

trait Unit {}

enum Utf16 {}

impl Encoding for Utf16 {
    type Unit = Utf16Unit;
    type FfiUnit = u16;
}

struct Utf16Unit(pub u16);

impl Unit for Utf16Unit {}

struct SUtf16Str {
    _data: Vec<Utf16Unit>,
}

trait FromPtr<E>: Sized where E: Encoding {
    unsafe fn from_ptr(ptr: <Slice as Structure<E>>::FfiPtr) -> Option<&'static Self>;
}

impl FromPtr<Utf16> for SUtf16Str {
    unsafe fn from_ptr(_: (*const u16, usize)) -> Option<&'static Self> {
        None
    }
}

fn main() {
    let TEXT_U16: [u16; 5] = [0, 1, 2, 3, 4];
    unsafe {
        let _ = SUtf16Str::from_ptr((TEXT_U16.as_ptr(), TEXT_U16.len()));
    }
}