#![feature(impl_trait_in_assoc_type)]
#![allow(dead_code)]

trait CFunction {
    type Output;
    extern "C" fn call() -> Self::Output;
}

trait CFunctionDebug: CFunction {
    fn debug_call(&self) -> String where Self::Output: std::fmt::Debug {
        format!("{:?}", Self::call())
    }
}

impl<T: CFunction> CFunctionDebug for T {}

struct ConcreteOpaque;

impl core::fmt::Debug for ConcreteOpaque {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConcreteOpaque")
    }
}

type Opaque = ConcreteOpaque;

impl CFunction for () {
    type Output = u32;
    extern "C" fn call() -> Self::Output {
        f()
    }
}

#[no_mangle]
extern "C" fn f() -> u32 {
    println!("Hello from C function!");
    42
}

pub fn main() {
    let _debug_output = <() as CFunctionDebug>::debug_call(&());
}