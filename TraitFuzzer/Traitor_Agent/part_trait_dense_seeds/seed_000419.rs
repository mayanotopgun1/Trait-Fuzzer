trait FooTrait {
    fn foo(&self) -> !;
}

impl FooTrait for () {
    fn foo(&self) -> ! {
        panic!("quux");
    }
}

fn main() {
    let _ = <() as FooTrait>::foo(&()) == <() as FooTrait>::foo(&());
}