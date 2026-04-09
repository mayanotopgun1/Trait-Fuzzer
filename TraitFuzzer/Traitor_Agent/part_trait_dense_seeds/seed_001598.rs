#[cfg(false)]
fn simple_attr<T>() where T: SimpleAttr {
    #[attr] if true {}
    #[allow_warnings] if true {}
}

trait SimpleAttr {}

impl SimpleAttr for () {}

#[cfg(false)]
fn if_else_chain<T>() where T: IfElseChain {
    #[first_attr] if true {
    } else if false {
    } else {
    }
}

trait IfElseChain {}

impl IfElseChain for () {}

#[cfg(false)]
fn if_let<T>() where T: IfLet {
    #[attr] if let Some(_) = Some(true) {}
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
    #[attr] if true {}
}

fn main() {}