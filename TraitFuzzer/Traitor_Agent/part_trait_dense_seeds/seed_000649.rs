trait Exit { fn exit(&self) -> !; }
impl Exit for () { fn exit(&self) -> ! { panic!() } }

fn f() -> ! {
    let _: &dyn Exit = &();
    ().exit()
}

fn g() -> isize {
    let x = match true {
        true => f(),
        false => 10,
    };
    return x;
}

fn main() {
    g();
}