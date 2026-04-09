trait Fooable { fn foo(&self) -> (u32, u32); }

impl Fooable for bool {
    fn foo(&self) -> (u32, u32) {
        if *self {
            return (5, 6);
        }
        let x: (u32, u32) = true.foo();
        println!("{:?}", x);
        (1u32, 2u32)
    }
}

fn main() {
    false.foo();
}