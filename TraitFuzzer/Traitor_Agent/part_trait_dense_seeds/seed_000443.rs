#![feature(unboxed_closures)]
#![feature(type_alias_impl_trait)]

type FunType = impl Fn<()>;

trait Call {
    type Output;
    fn call(&self) -> Self::Output;
}

impl<F: Fn<()>> Call for F {
    type Output = <F as FnOnce<()>>::Output;
    fn call(&self) -> Self::Output {
        self()
    }
}

trait CallExt: Call {}
impl<T: Call> CallExt for T {}

#[define_opaque(FunType)]
fn foo() -> FunType {
    some_fn
}

fn some_fn() {}

fn main() {
    let f: FunType = foo();
    let _: <FunType as Call>::Output = f.call();
}