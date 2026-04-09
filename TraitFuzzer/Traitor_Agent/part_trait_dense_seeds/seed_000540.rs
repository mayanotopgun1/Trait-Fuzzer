#![feature(type_alias_impl_trait)]

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

type SUtf16Str = SeStr<Slice, Utf16>;

struct SeStr<S, E> where S: Structure<E>, E: Encoding {
    _data: *const <S as Structure<E>>::RefTarget,
}

trait SeStrExt<S, E>: Sized where S: Structure<E>, E: Encoding {
    unsafe fn create_from_ptr<'a>(ptr: S::FfiPtr) -> Option<&'a Self>;
}

impl<S, E> SeStrExt<S, E> for SeStr<S, E>
where
    S: Structure<E>,
    E: Encoding,
{
    unsafe fn create_from_ptr<'a>(_ptr: S::FfiPtr) -> Option<&'a Self> {
        panic!()
    }
}

trait SeStrUtils<S, E>: SeStrExt<S, E> where S: Structure<E>, E: Encoding {
    fn is_empty(&self) -> bool;
}

impl<S, E> SeStrUtils<S, E> for SeStr<S, E>
where
    S: Structure<E>,
    E: Encoding,
{
    fn is_empty(&self) -> bool {
        self._data.is_null()
    }
}

fn main() {
    const TEXT_U16: &'static [u16] = &[];
    let se_str = unsafe { <SeStr<Slice, Utf16> as SeStrExt<Slice, Utf16>>::create_from_ptr((TEXT_U16.as_ptr(), TEXT_U16.len())).unwrap() };
    println!("Is empty: {}", se_str.is_empty());
}