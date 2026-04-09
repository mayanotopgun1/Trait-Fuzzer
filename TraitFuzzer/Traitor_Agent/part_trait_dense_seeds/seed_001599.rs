#![feature(impl_trait_in_assoc_type)]

#[cfg(false)]
fn simple_attr<T>() -> impl SimpleAttr where T: SimpleAttr {
    if true {}
    if true {}
}

trait SimpleAttr {}

impl SimpleAttr for () {}

#[cfg(false)]
fn if_else_chain<T>() -> impl IfElseChain where T: IfElseChain {
    if true {
    } else if false {
    } else {
    }
}

trait IfElseChain {}

impl IfElseChain for () {}

#[cfg(false)]
fn if_let<T>() -> impl IfLet where T: IfLet {
    if let Some(_) = Some(true) {}
}

trait IfLet {}

impl IfLet for () {}

fn bar() {
    #[cfg(false)]
    if true {
        let x: () = true;
    }

    #[cfg_attr(not(FALSE), cfg(false))]
    if true {
        let a: () = true;
    }
}

macro_rules! custom_macro {
    ($expr:expr) => {}
}

custom_macro! {
    if true {}
}

fn main() {}