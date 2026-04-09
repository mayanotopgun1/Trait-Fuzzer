use std::ptr;

extern "C" fn c_fn() {}

fn static_i32() -> &'static i32 {
    &42
}

trait PtrUtils {
    type Output;
    fn is_null(&self) -> Self::Output;
}

impl<T> PtrUtils for *const T {
    type Output = bool;
    fn is_null(&self) -> Self::Output {
        self.is_null()
    }
}

impl<T> PtrUtils for *mut T {
    type Output = bool;
    fn is_null(&self) -> Self::Output {
        self.is_null()
    }
}

fn main() {
    let fn_ptr: *const () = main as *const ();
    let c_fn_ptr: *const () = c_fn as *const ();
    let mut_ref_ptr: *mut i32 = &mut 8;
    let ref_ptr: *const i32 = &8;
    let slice_ptr: *const [i32; 2] = &[1, 2];
    let mut_slice_ptr: *mut [i32; 2] = &mut [1, 2];
    let static_i32_ptr: *const i32 = static_i32();

    assert!(!fn_ptr.is_null());
    assert!(!c_fn_ptr.is_null());
    assert!(!mut_ref_ptr.is_null());
    assert!(!ref_ptr.is_null());
    assert!(!slice_ptr.is_null());
    assert!(!mut_slice_ptr.is_null());
    assert!(!static_i32_ptr.is_null());

    const ZPTR: *const () = std::ptr::null();
    const NOT_ZPTR: *const i32 = 1 as *const i32;

    assert!(ZPTR.is_null());
    assert!(!NOT_ZPTR.is_null());
}