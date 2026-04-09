#![feature(type_alias_impl_trait)]

trait Exit { fn exit(&self) -> !; }
impl Exit for () { fn exit(&self) -> ! { panic!() } }

fn f() -> impl Exit {
    let _: &dyn Exit = &();
    ()
}

fn g() -> isize {
    match true {
        true => { f().exit(); 0 },
        false => 10,
    }
}

fn main() {
    g();
}