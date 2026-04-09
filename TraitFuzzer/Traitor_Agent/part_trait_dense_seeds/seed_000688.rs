#![feature(impl_trait_in_assoc_type)]

#[derive(Debug)]
struct VTable<DST: ?Sized> {
    _to_dst_ptr: fn(*mut ()) -> *mut DST,
}

trait HasVTableFor<DST: ?Sized + 'static> {
    const VTABLE: &'static VTable<DST>;
}

impl<T, DST: ?Sized + 'static> HasVTableFor<DST> for T {
    const VTABLE: &'static VTable<DST> = &VTable {
        _to_dst_ptr: |_: *mut ()| unsafe { std::mem::zeroed() },
    };
}

trait VTablePush {
    fn push_vtable<DST: ?Sized + 'static>(self) -> impl core::fmt::Debug where Self: HasVTableFor<DST>, DST: std::fmt::Debug;
}

impl<T> VTablePush for T {
    fn push_vtable<DST: ?Sized + 'static>(self) -> impl core::fmt::Debug where Self: HasVTableFor<DST>, DST: std::fmt::Debug {
        <Self as HasVTableFor<DST>>::VTABLE
    }
}

pub fn push<DST: ?Sized + 'static, T>(t: T) -> impl core::fmt::Debug where DST: std::fmt::Debug {
    t.push_vtable::<DST>()
}

fn main() {}