#![feature(type_alias_impl_trait)]

#[target_feature(enable = "sse2")]
unsafe fn sse2() {}

trait Sse2Operations {
    type Out;
    unsafe fn sse2(&self) -> Self::Out;
}

impl Sse2Operations for Foo {
    type Out = u32;
    unsafe fn sse2(&self) -> Self::Out {
        sse2();
        42u32
    }
}

struct Foo;

trait Sse2Execution {
    unsafe fn execute_sse2(&self);
}

impl<T: ?Sized + Sse2Operations> Sse2Execution for T {
    unsafe fn execute_sse2(&self) {
        let _result = self.sse2();
    }
}

fn main() {
    if cfg!(target_feature = "sse2") {
        unsafe {
            sse2();
            let foo = Foo;
            (&foo as &dyn Sse2Operations<Out = u32>).execute_sse2();
        }
    }
    let _sse2_ptr: unsafe fn() = sse2;
}